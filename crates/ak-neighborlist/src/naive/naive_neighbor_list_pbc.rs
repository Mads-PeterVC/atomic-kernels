use ak_core::StructureView;
use nalgebra::Vector3;

use crate::NeighborList;

pub(super) fn naive_neighbor_list_pbc(view: &StructureView, cutoff: f64) -> NeighborList {
    let n_atoms = view.len();

    let mut i_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut j_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut shift: Vec<[i32; 3]> = Vec::with_capacity(4 * n_atoms);
    let mut distances: Vec<f64> = Vec::with_capacity(4 * n_atoms);

    // Compute relevant shifts:
    let cell_vectors = [view.cell.a(), view.cell.b(), view.cell.c()];

    let repeat_counts: [i32; 3] = std::array::from_fn(|i| {
        if view.pbc[i] {
            ((cutoff / cell_vectors[i].norm()).ceil() + 1.0) as i32
        } else {
            0
        }
    });

    let positions = view.positions;

    for (i, position_i) in positions.iter().enumerate().take(n_atoms) {
        let pos_i = Vector3::new(position_i[0], position_i[1], position_i[2]);

        for (j, position_j) in positions.iter().enumerate().take(n_atoms) {
            let pos_j = Vector3::new(position_j[0], position_j[1], position_j[2]);

            for shift_a in -repeat_counts[0]..=repeat_counts[0] {
                for shift_b in -repeat_counts[1]..=repeat_counts[1] {
                    for shift_c in -repeat_counts[2]..=repeat_counts[2] {
                        if i == j && shift_a == 0 && shift_b == 0 && shift_c == 0 {
                            continue;
                        }

                        let shift_vec = cell_vectors[0] * (shift_a as f64)
                            + cell_vectors[1] * (shift_b as f64)
                            + cell_vectors[2] * (shift_c as f64);

                        let diff = pos_i - pos_j - shift_vec;

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
