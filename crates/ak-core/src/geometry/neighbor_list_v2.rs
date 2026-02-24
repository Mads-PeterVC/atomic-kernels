use crate::{NeighborList, StructureView};
use nalgebra::Vector3;

pub fn build_neighborlist(view: &StructureView, cutoff: f64) -> NeighborList {
    let n_atoms = view.len();

    let mut i_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut j_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut shift: Vec<[i32; 3]> = Vec::with_capacity(4 * n_atoms);
    let mut distances: Vec<f64> = Vec::with_capacity(4 * n_atoms);

    // Compute relevant shifts:
    let cell_vectors = [view.cell.a(), view.cell.b(), view.cell.c()];

    let repeat_counts: [i32; 3] = std::array::from_fn(|i| {
        if view.pbc.0[i] {
            (cell_vectors[i].norm() / cutoff).ceil() as i32
        } else {
            0
        }
    });

    let positions = view.positions;

    for i in 0..n_atoms {
        let pos_i = Vector3::new(positions[i][0], positions[i][1], positions[i][2]);

        for j in (i + 1)..n_atoms {
            let pos_j = Vector3::new(positions[j][0], positions[j][1], positions[j][2]);

            for shift_a in -repeat_counts[0]..=repeat_counts[0] {
                for shift_b in -repeat_counts[1]..=repeat_counts[1] {
                    for shift_c in -repeat_counts[2]..=repeat_counts[2] {
                        let shift_vec = cell_vectors[0] * (shift_a as f64)
                            + cell_vectors[1] * (shift_b as f64)
                            + cell_vectors[2] * (shift_c as f64);

                        let diff = pos_i - pos_j + shift_vec;

                        let norm = diff.norm();

                        if norm < cutoff {
                            i_indices.push(i);
                            j_indices.push(j);
                            shift.push([shift_a, shift_b, shift_c]);
                            distances.push(norm);
                        }
                    }
                }
            }
        }
    }

    NeighborList {
        i: i_indices,
        j: j_indices,
        shifts: shift,
        distance: Some(distances),
    }
}

#[cfg(test)]
mod test {

    use crate::Structure;
    use crate::geometry::neighbor_list_v2::build_neighborlist;

    fn test_structure() -> Structure {
        let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let numbers = vec![1, 1];
        let cell = [[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]];
        let pbc = [true, false, false];
        Structure::new(positions, numbers, cell, pbc)
    }
    #[test]
    fn test() {
        let structure = test_structure();
        let nl = build_neighborlist(&structure.view(), 1.5);
        assert_eq!(nl.i.len(), 1)
    }
}
