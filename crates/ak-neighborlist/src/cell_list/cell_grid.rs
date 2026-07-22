use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CellGridError {
    #[error("Cell width has to be positive and finite")]
    InvalidCellWidth,
    #[error("The number of cell is too large, leads to overflow")]
    CellOverflow,
    #[error("The number of cells in each dimension has to be at least 1")]
    CellCount,
    #[error("Origin must be finite")]
    InvalidOrigin,
    #[error("Coordinate must be finite")]
    InvalidCoordinate,
    #[error("Position is outside of the grid")]
    PositionOutsideGrid { position: [f64; 3] },
    #[error("Position led to non-finite cell coordinate")]
    NonfiniteCellCoordinate { position: [f64; 3] },
}

#[derive(Debug, PartialEq)]
pub(super) struct CellGrid {
    origin: [f64; 3],
    cell_width: [f64; 3],
    n_cells: [usize; 3],
    cells: Vec<Vec<usize>>,
}

pub(super) struct NeighborCell<'a> {
    pub(super) index: usize,
    pub(super) atoms: &'a [usize],
    pub(super) shift: [i32; 3],
}

impl CellGrid {
    pub(super) fn new(
        origin: [f64; 3],
        cell_width: [f64; 3],
        n_cells: [usize; 3],
    ) -> Result<CellGrid, CellGridError> {
        if origin.iter().any(|x| x.is_infinite() || x.is_nan()) {
            return Err(CellGridError::InvalidOrigin);
        }

        if n_cells.iter().any(|x| x < &1) {
            return Err(CellGridError::CellCount);
        }

        if cell_width.iter().any(|c| !c.is_finite() || *c <= 0.0) {
            return Err(CellGridError::InvalidCellWidth);
        }

        let total_cells = n_cells[0]
            .checked_mul(n_cells[1])
            .and_then(|xy| xy.checked_mul(n_cells[2]))
            .ok_or(CellGridError::CellOverflow)?;

        let cells = vec![Vec::new(); total_cells];

        Ok(CellGrid {
            origin: origin,
            cell_width: cell_width,
            n_cells: n_cells,
            cells: cells,
        })
    }

    fn cell_coordinate(&self, position: &[f64; 3]) -> Result<[usize; 3], CellGridError> {
        if position.iter().any(|p| p.is_infinite() || p.is_nan()) {
            return Err(CellGridError::InvalidCoordinate);
        }

        let cx = f64::floor((position[0] - self.origin[0]) / self.cell_width[0]);
        let cy = f64::floor((position[1] - self.origin[1]) / self.cell_width[1]);
        let cz = f64::floor((position[2] - self.origin[2]) / self.cell_width[2]);

        for (i, c) in [cx, cy, cz].iter().enumerate() {
            if !c.is_finite() {
                return Err(CellGridError::NonfiniteCellCoordinate {
                    position: *position,
                });
            }

            if !(0.0 <= *c && *c < (self.n_cells[i] as f64)) {
                return Err(CellGridError::PositionOutsideGrid {
                    position: *position,
                });
            }
        }

        Ok([cx as usize, cy as usize, cz as usize])
    }

    fn linear_index(&self, c: [usize; 3]) -> usize {
        c[0] + self.n_cells[0] * (c[1] + self.n_cells[1] * c[2])
    }

    fn convert_linear_index(&self, index: &usize) -> [usize; 3] {
        let xy = self.n_cells[0] * self.n_cells[1];
        let z = index / xy;
        let remainder = index % xy;
        let y = remainder / self.n_cells[0];
        let x = remainder % self.n_cells[0];
        [x, y, z]
    }

    pub(super) fn assign(
        &mut self,
        atom_index: usize,
        position: &[f64; 3],
    ) -> Result<(), CellGridError> {
        let cell_coords = self.cell_coordinate(position)?;
        let linear_index = self.linear_index(cell_coords);
        self.cells[linear_index].push(atom_index);
        Ok(())
    }

    pub(super) fn shape(&self) -> [usize; 3] {
        self.n_cells
    }

    pub(super) fn total_cells(&self) -> usize {
        self.n_cells[0] * self.n_cells[1] * self.n_cells[2]
    }

    pub(super) fn cells(&self) -> &Vec<Vec<usize>> {
        &self.cells
    }

    pub(super) fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub(super) fn neighboring_cells(&self, cell_index: usize) -> Vec<NeighborCell<'_>> {
        let cc = self.convert_linear_index(&cell_index);
        let mut neighbor_cells = Vec::with_capacity(26);

        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                for dz in -1isize..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }

                    let Some(neighbor_x) = cc[0].checked_add_signed(dx) else {
                        continue;
                    };
                    let Some(neighbor_y) = cc[1].checked_add_signed(dy) else {
                        continue;
                    };
                    let Some(neighbor_z) = cc[2].checked_add_signed(dz) else {
                        continue;
                    };

                    if neighbor_x >= self.n_cells[0]
                        || neighbor_y >= self.n_cells[1]
                        || neighbor_z >= self.n_cells[2]
                    {
                        continue;
                    }

                    let neighbor_index = self.linear_index([neighbor_x, neighbor_y, neighbor_z]);

                    neighbor_cells.push(NeighborCell {
                        index: neighbor_index,
                        atoms: self.cells[neighbor_index].as_slice(),
                        shift: [0, 0, 0],
                    });
                }
            }
        }

        neighbor_cells
    }
}

#[cfg(test)]
mod test {

    use super::{CellGrid, CellGridError};

    fn cell_grid() -> CellGrid {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, 2.5];
        let n_cells = [3, 3, 3];
        CellGrid::new(origin, width, n_cells).unwrap()
    }

    #[test]
    fn test_cell_grid_origin_nan() {
        let origin = [0.0, 0.0, f64::NAN];
        let width = [2.5, 2.5, 2.5];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidOrigin)
        )
    }

    #[test]
    fn test_cell_grid_origin_nonfinite() {
        let origin = [0.0, 0.0, f64::INFINITY];
        let width = [2.5, 2.5, 2.5];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidOrigin)
        )
    }

    #[test]
    fn test_cell_width_finite_zero() {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, 0.0];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidCellWidth)
        )
    }

    #[test]
    fn test_cell_width_finite_negative() {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, -1.0];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidCellWidth)
        )
    }

    #[test]
    fn test_cell_width_finite_nan() {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, f64::NAN];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidCellWidth)
        )
    }

    #[test]
    fn test_cell_width_finite_inf() {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, f64::INFINITY];
        let n_cells = [3, 3, 3];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::InvalidCellWidth)
        )
    }

    #[test]
    fn test_cell_grid_overflow() {
        let origin = [0.0, 0.0, 0.0];
        let width = [2.5, 2.5, 2.5];
        let n_cells = [usize::MAX - 1, usize::MAX - 1, usize::MAX - 1];
        assert_eq!(
            CellGrid::new(origin, width, n_cells),
            Err(CellGridError::CellOverflow)
        )
    }

    #[test]
    fn test_cell_grid_init() {
        let grid = cell_grid();
        assert_eq!(grid.cells.len(), 27);
        assert!(grid.cells.iter().all(Vec::is_empty));
    }

    #[test]
    fn test_cell_grid() {
        let mut cell_grid = cell_grid();

        let positions = [[1.0, 1.0, 1.0], [2.5, 0.0, 0.0]];

        for (atom_index, position) in positions.iter().enumerate() {
            cell_grid.assign(atom_index, position).unwrap();
        }

        assert!(cell_grid.cells()[0].contains(&0));
        assert!(cell_grid.cells()[1].contains(&1));
    }

    #[test]
    fn test_cell_width_uneven() {
        let grid = CellGrid::new([0.0, 0.0, 0.0], [2.0, 4.0, 5.0], [3, 3, 3]).unwrap();
        let position = [2.1, 4.1, 5.1];
        assert_eq!(grid.cell_coordinate(&position), Ok([1, 1, 1]))
    }

    #[test]
    fn test_upper_boundary_exclusive() {
        let mut grid = cell_grid();

        assert_eq!(
            grid.assign(0, &[7.5, 7.5, 7.5]),
            Err(CellGridError::PositionOutsideGrid {
                position: [7.5, 7.5, 7.5]
            })
        );
    }

    #[test]
    fn test_near_upper_boundary() {
        let mut grid = cell_grid();
        let offset = 0.00000001;
        assert_eq!(
            grid.assign(0, &[7.5 - offset, 7.5 - offset, 7.5 - offset]),
            Ok(())
        );
    }

    #[test]
    fn test_out_of_bounds_negative() {
        let mut cell_grid = cell_grid();
        let positions = [[-1.0, -1.0, -1.0]];
        assert_eq!(
            cell_grid.assign(0, &positions[0]),
            Err(CellGridError::PositionOutsideGrid {
                position: positions[0]
            })
        );
    }

    #[test]
    fn test_out_of_bounds_positive() {
        let mut cell_grid = cell_grid();
        let positions = [[10.0, 10.0, 10.0]];
        assert_eq!(
            cell_grid.assign(0, &positions[0]),
            Err(CellGridError::PositionOutsideGrid {
                position: positions[0]
            })
        );
    }

    #[test]
    fn test_invalid_coordinate() {
        let mut cell_grid = cell_grid();
        let positions = [[10.0, 10.0, f64::NAN]];

        assert_eq!(
            cell_grid.assign(0, &positions[0]),
            Err(CellGridError::InvalidCoordinate)
        );
    }
}
