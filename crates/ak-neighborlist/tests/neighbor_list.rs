use ak_core::Structure;
use ak_neighborlist::{naive_neighbor_list, naive_neighbor_list_pbc};

fn example_structure() -> Structure {
    Structure::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        vec![1, 8, 1],
        [[4.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 6.0]],
        [false, false, false],
    )
}

#[test]
fn non_periodic_neighbor_list_finds_pairs_with_distances() {
    let structure = example_structure();
    let neighbors = naive_neighbor_list(&structure.view(), 1.1);

    assert_eq!(neighbors.i, vec![0, 0]);
    assert_eq!(neighbors.j, vec![1, 2]);
    assert_eq!(neighbors.shifts, vec![[0, 0, 0], [0, 0, 0]]);
    assert_eq!(neighbors.distance.unwrap(), vec![1.0, 1.0]);
}

#[test]
fn periodic_neighbor_list_finds_wrapped_neighbor() {
    let periodic = Structure::new(
        vec![[0.0, 0.0, 0.0], [2.7, 0.0, 0.0]],
        vec![1, 1],
        [[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]],
        [true, false, false],
    );

    let neighbors = naive_neighbor_list_pbc(&periodic.view(), 0.5);

    assert_eq!(neighbors.i, vec![0, 1]);
    assert_eq!(neighbors.j, vec![1, 0]);
    assert_eq!(neighbors.shifts, vec![[-1, 0, 0], [1, 0, 0]]);
}
