use crate::StructureView;

pub struct NeighborList {
    pub i: Vec<usize>,
    pub j: Vec<usize>,
    pub shifts: Vec<[i32; 3]>,
    pub distance: Option<Vec<f64>>,
}

fn squared_distance(positions: &[[f64; 3]], i: usize, j: usize) -> f64 {
    let dx = positions[i][0] - positions[j][0];
    let dy = positions[i][1] - positions[j][1];
    let dz = positions[i][2] - positions[j][2];
    let r2 = dx * dx + dy * dy + dz * dz;
    r2
}

pub fn build_neighborlist(view: &StructureView, cutoff: f64) -> NeighborList {
    let n_atoms = view.positions.len();

    let mut i_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut j_indices: Vec<usize> = Vec::with_capacity(4 * n_atoms);
    let mut shift: Vec<[i32; 3]> = Vec::with_capacity(4 * n_atoms);
    let mut distances: Vec<f64> = Vec::with_capacity(4 * n_atoms);

    let n_atoms = view.positions.len();
    let squared_cutoff = cutoff * cutoff;

    for i in 0..n_atoms {
        for j in (i + 1)..n_atoms {
            let r2 = squared_distance(view.positions, i, j);
            if r2 <= squared_cutoff {
                i_indices.push(i);
                j_indices.push(j);
                shift.push([0, 0, 0]);
                distances.push(r2.sqrt());
            }
        }
    }

    let nl = NeighborList {
        i: i_indices,
        j: j_indices,
        shifts: shift,
        distance: Some(distances),
    };
    nl
}
