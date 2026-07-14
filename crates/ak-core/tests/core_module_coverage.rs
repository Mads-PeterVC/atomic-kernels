use std::io::{BufReader, Cursor};

use ak_core::{AtomicNumber, Cell, PERIODIC_TABLE, Pbc, Structure, Trajectory, distance_matrix};
use nalgebra::Vector3;

fn example_structure() -> Structure {
    Structure::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        vec![1, 8, 1],
        [[4.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 6.0]],
        [false, false, false],
    )
}

#[test]
fn atomic_number_validates_range() {
    assert_eq!(AtomicNumber::new(8).unwrap().get(), 8);
    assert!(AtomicNumber::new(0).is_err());
    assert!(AtomicNumber::new(111).is_err());
}

#[test]
fn cell_exposes_vectors_and_reduction() {
    let cell = Cell::new([[4.0, 0.0, 0.0], [0.0, 5.0, 0.0], [1.0, 0.0, 6.0]]);

    assert!(!cell.is_orthorhombic());
    assert_eq!(cell.a(), Vector3::new(4.0, 0.0, 0.0));
    assert_eq!(cell.b(), Vector3::new(0.0, 5.0, 0.0));
    assert_eq!(cell.c(), Vector3::new(1.0, 0.0, 6.0));
    assert_eq!(cell.reduced(0.5, 0.5, 0.0), Vector3::new(2.0, 2.5, 0.0));
}

#[test]
fn pbc_reports_any_and_all() {
    let partial = Pbc::new([true, false, false]);
    let full = Pbc::new([true, true, true]);

    assert!(partial.any());
    assert!(!partial.all());
    assert!(full.all());
}

#[test]
fn structure_and_xyz_reader_build_equivalent_views() {
    let xyz = r#"3
Lattice="4.0 0.0 0.0 0.0 5.0 0.0 0.0 0.0 6.0" Properties=species:S:1:pos:R:3 pbc="T F F"
H 0.0 0.0 0.0
O 1.0 0.0 0.0
H 0.0 1.0 0.0
"#;
    let parsed = Structure::from_xyz_reader(BufReader::new(Cursor::new(xyz)));
    let manual = example_structure();
    let view = parsed.view();

    assert_eq!(view.positions, manual.view().positions);
    assert_eq!(view.numbers[1].get(), 8);
    assert_eq!(view.cell.a(), Vector3::new(4.0, 0.0, 0.0));
    assert_eq!(*view.pbc, [true, false, false]);
}

#[test]
fn trajectory_append_and_view_work() {
    let first = example_structure();
    let mut trajectory = Trajectory::new(vec![first.clone()]);

    trajectory.append(example_structure());

    assert_eq!(trajectory.len(), 2);
    assert!(!trajectory.is_empty());
    assert_eq!(trajectory.view(1).positions[2], [0.0, 1.0, 0.0]);
}

#[test]
fn distance_matrix_covers_geometry_helpers() {
    let structure = example_structure();
    let view = structure.view();

    let distances = distance_matrix(&view);

    assert_eq!(distances.len(), 9);
    assert_eq!(distances[1], 1.0);
}

#[test]
fn periodic_table_exposes_consistent_element_data() {
    let hydrogen = PERIODIC_TABLE.get(AtomicNumber::new(1).unwrap());
    let oxygen = PERIODIC_TABLE.get_by_symbol("O");

    assert_eq!(hydrogen.symbol, "H");
    assert_eq!(oxygen.number.get(), 8);
    assert!(oxygen.covalent_radius > 0.0);
}
