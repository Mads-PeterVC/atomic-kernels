use crate::cell_list::grid_builder::build_non_periodic_grid;
use crate::nl::{NeighborList, NeighborListError};
use ak_core::StructureView;

fn distance_square(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    (p1[0] - p2[0]).powf(2.0) + (p1[1] - p2[1]).powf(2.0) + (p1[2] - p2[2]).powf(2.0)
}

pub fn cell_list_neighborlist(
    view: &StructureView,
    cutoff: f64,
) -> Result<NeighborList, NeighborListError> {
    let grid = if !view.pbc.any() {
        build_non_periodic_grid(view, cutoff).map_err(|_| NeighborListError::CatchAllError)?
    } else {
        todo!("Periodic not yet implemented")
    };

    let mut index_i: Vec<usize> = Vec::new();
    let mut index_j: Vec<usize> = Vec::new();
    let cutoff2 = cutoff.powf(2.0);

    for cell_index in 0..grid.total_cells() {
        let atoms = &grid.cells()[cell_index];
        if atoms.is_empty() {
            continue;
        }
        for (pos, i) in atoms.iter().enumerate() {
            for j in atoms.iter().skip(pos + 1) {
                let d2 = distance_square(&view.positions[*i], &view.positions[*j]);
                if d2 < cutoff2 {
                    index_i.push(*i);
                    index_j.push(*j);
                }
            }
        }

        for neighbor in grid.neighboring_cells(cell_index) {
            if neighbor.index <= cell_index {
                continue;
            }

            for i in atoms {
                for j in neighbor.atoms {
                    if distance_square(&view.positions[*i], &view.positions[*j]) <= cutoff2 {
                        index_i.push(*i);
                        index_j.push(*j);
                    }
                }
            }
        }
    }

    let nl = NeighborList {
        i: index_i,
        j: index_j,
        shifts: Vec::new(),
        distance: None,
    };
    Ok(nl)
}

#[cfg(test)]
mod test {

    use ak_core::Structure;

    use crate::cell_list::pair_traversal::cell_list_neighborlist;

    fn test_structure(positions: Vec<[f64; 3]>) -> Structure {
        let numbers = vec![1_i32; positions.len()];
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        Structure::new(positions, numbers, cell, pbc)
    }

    #[test]
    fn test_neighborlist_simple() {
        let positions = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [3.0, 1.0, 1.0]];
        let structure = test_structure(positions);
        let nl = cell_list_neighborlist(&structure.view(), 2.5).unwrap();
        assert_eq!(nl.i, [0, 1]);
        assert_eq!(nl.j, [1, 2]);
    }
}
