use crate::cell_list::cell_grid::{CellGrid, CellGridError};
use ak_core::StructureView;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(super) enum BuildGridError {
    #[error("The passed structure was empty")]
    EmptyStructure,
    #[error("Grid construction failed")]
    CellGridError { error: CellGridError },
    #[error("Cutoff must be positive and finite")]
    InvalidCutoff,
}

pub(super) fn build_non_periodic_grid(
    view: &StructureView,
    cutoff: f64,
) -> Result<CellGrid, BuildGridError> {
    // Construct origin as minimum

    if cutoff <= 0.0 || !cutoff.is_finite() {
        return Err(BuildGridError::InvalidCutoff);
    }

    let minimum = view
        .positions
        .iter()
        .copied()
        .reduce(|minima, value| {
            [
                minima[0].min(value[0]),
                minima[1].min(value[1]),
                minima[2].min(value[2]),
            ]
        })
        .ok_or(BuildGridError::EmptyStructure)?;

    let maximum = view
        .positions
        .iter()
        .copied()
        .reduce(|maxima, value| {
            [
                maxima[0].max(value[0]),
                maxima[1].max(value[1]),
                maxima[2].max(value[2]),
            ]
        })
        .ok_or(BuildGridError::EmptyStructure)?;

    let cell_width = [cutoff, cutoff, cutoff];

    let cells_x = ((maximum[0] - minimum[0]) / cutoff).floor() + 1.0;
    let cells_y = ((maximum[1] - minimum[1]) / cutoff).floor() + 1.0;
    let cells_z = ((maximum[2] - minimum[2]) / cutoff).floor() + 1.0;

    for c in [cells_x, cells_y, cells_z] {
        if c >= usize::MAX as f64 || !c.is_finite() {
            return Err(BuildGridError::CellGridError {
                error: CellGridError::CellCount,
            });
        }
    }

    let mut grid = CellGrid::new(
        minimum,
        cell_width,
        [cells_x as usize, cells_y as usize, cells_z as usize],
    )
    .map_err(|x| BuildGridError::CellGridError { error: x })?;

    for (atom_index, position) in view.positions.iter().enumerate() {
        grid.assign(atom_index, position)
            .map_err(|x| BuildGridError::CellGridError { error: x })?;
    }

    Ok(grid)
}

#[cfg(test)]
mod test {

    use crate::cell_list::grid_builder::BuildGridError;

    use super::build_non_periodic_grid;
    use ak_core::Structure;

    fn test_structure(positions: Vec<[f64; 3]>) -> Structure {
        let numbers = vec![1_i32; positions.len()];
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        Structure::new(positions.clone(), numbers.clone(), cell, pbc)
    }

    #[test]
    fn test_build_non_pbc_1x1x1() {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]].to_vec();
        let structure = test_structure(positions);
        let cutoff = 2.5;
        let grid =
            build_non_periodic_grid(&structure.view(), cutoff).expect("This should not fail");
        assert_eq!(grid.shape(), [1, 1, 1]);
        assert_eq!(grid.cells()[0], [0, 1]);
    }

    #[test]
    fn test_build_non_pbc_2x1x1() {
        let positions = [[0.0, 0.0, 0.0], [2.51, 0.0, 0.0]].to_vec();
        let structure = test_structure(positions);
        let cutoff = 2.5;
        let grid =
            build_non_periodic_grid(&structure.view(), cutoff).expect("This should not fail");
        assert_eq!(grid.shape(), [2, 1, 1]);
        assert_eq!(grid.cells()[0], [0]);
        assert_eq!(grid.cells()[1], [1]);

        assert_eq!(
            grid.cells()
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<usize>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn test_build_non_pbc_many() {
        let mut positions = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    positions.push([i as f64, j as f64, k as f64])
                }
            }
        }
        let structure = test_structure(positions);
        let cutoff = 2.5;
        let grid =
            build_non_periodic_grid(&structure.view(), cutoff).expect("This should not fail");
        assert_eq!(grid.shape(), [4, 4, 4]);

        let mut assignments = grid
            .cells()
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<usize>>();

        assignments.sort();

        assert_eq!(assignments, Vec::from_iter(0..1000));
    }

    #[test]
    fn test_build_non_pbc_negative_coordinates() {
        let positions = vec![[0.0, 0.0, 0.0], [-1.0, -1.0, -1.0]];
        let structure = test_structure(positions);
        let grid = build_non_periodic_grid(&structure.view(), 2.5).expect("Should not fail");
        assert_eq!(grid.shape(), [1, 1, 1]);
        assert_eq!(grid.origin(), [-1.0, -1.0, -1.0]);
        assert_eq!(grid.cells()[0], [0, 1]);
    }

    #[test]
    fn test_build_non_pbc_empty() {
        let positions: Vec<[f64; 3]> = Vec::new();
        let structure = test_structure(positions);
        let grid = build_non_periodic_grid(&structure.view(), 2.5);
        assert_eq!(grid, Err(BuildGridError::EmptyStructure));
    }
}
