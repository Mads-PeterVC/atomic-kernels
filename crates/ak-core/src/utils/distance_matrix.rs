use crate::StructureView;

pub fn distance_matrix(view: &StructureView) -> Vec<f64> {
    let positions = view.positions;
    let n = positions.len();
    let mut output = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            let dx = positions[i][0] - positions[j][0];
            let dy = positions[i][1] - positions[j][1];
            let dz = positions[i][2] - positions[j][2];
            let r2 = dx * dx + dy * dy + dz * dz;
            output.push(f64::sqrt(r2));
        }
    }

    output
}

#[test]
fn test_shape() {
    use crate::Structure;
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]].to_vec();
    let numbers = [1, 1, 1].to_vec();
    let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
    let pbc = [false, false, false];
    let structure = Structure::new(positions.clone(), numbers.clone(), cell, pbc);
    let view = structure.view();

    let distances = distance_matrix(&view);
    println!("{:#?}", distances);
    assert_eq!(distances.len(), 9)
}

#[test]
fn test_output() {
    use crate::Structure;
    use std::f64::consts::SQRT_2;

    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]].to_vec();
    let numbers = [1, 1, 1].to_vec();
    let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
    let pbc = [false, false, false];
    let structure = Structure::new(positions.clone(), numbers.clone(), cell, pbc);
    let view = structure.view();
    let distances = distance_matrix(&view);

    let expected_distances = vec![0.0, 1.0, 1.0, 1.0, 0.0, SQRT_2, 1.0, SQRT_2, 0.0];
    assert_eq!(expected_distances, distances)
}
