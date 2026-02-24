use nalgebra::{Matrix3, Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    // Rows are lattice vectors a, b, c (stored column-major internally by nalgebra)
    pub m: Matrix3<f64>,
}

impl Cell {
    pub fn new(cell: [[f64; 3]; 3]) -> Self {
        Cell {
            m: Matrix3::from_row_slice(cell.as_flattened()),
        }
    }

    pub fn is_orthorhombic(&self) -> bool {
        let eps = 1e-12;

        // Subtract the diagonal part — remainder should be ~zero
        let off_diagonal = self.m - Matrix3::from_diagonal(&self.m.diagonal());

        // All off-diagonal elements are ~0
        let is_diagonal = off_diagonal.abs().max() < eps;

        // All diagonal elements are positive
        let has_positive_diagonal = self.m.diagonal().iter().all(|&x| x > eps);

        is_diagonal && has_positive_diagonal
    }

    pub fn a(&self) -> Vector3<f64> {
        self.m.row(0).transpose()
    }

    pub fn b(&self) -> Vector3<f64> {
        self.m.row(1).transpose()
    }

    pub fn c(&self) -> Vector3<f64> {
        self.m.row(2).transpose()
    }

    pub fn reduced(&self, a: f64, b: f64, c: f64) -> Vector3<f64> {
        self.m.transpose() * Vector3::new(a, b, c)
    }
}

#[cfg(test)]
mod tests {

    use crate::Cell;

    #[test]
    fn test_ortho() {
        let cell = Cell::new([[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]]);
        assert_eq!(cell.is_orthorhombic(), true)
    }

    #[test]
    fn test_not_ortho() {
        let cell = Cell::new([[10.0, 5.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]]);
        assert_eq!(cell.is_orthorhombic(), false)
    }
}
