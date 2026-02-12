#[derive(Clone, Copy, Debug)]
pub struct Cell {
    // Row-major 3x3
    pub m: [[f64; 3]; 3],
}

impl Cell {
    pub fn new(cell: [[f64; 3]; 3]) -> Self {
        Cell { m: cell }
    }

    pub fn is_orthorhombic(&self) -> bool {
        let eps = 1e-12;

        for i in 0..3 {
            for j in 0..3 {
                let v = self.m[i][j];

                if i == j {
                    if v <= eps {
                        return false;
                    }
                } else {
                    if v > eps {
                        return false;
                    }
                }
            }
        }
        true
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
