#[derive(Clone, Copy, Debug, PartialEq)]
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
                } else if v > eps {
                    return false;
                }
            }
        }
        true
    }

    pub fn a(&self) -> [f64; 3] {
        self.m[0]
    }

    pub fn b(&self) -> [f64; 3] {
        self.m[1]
    }

    pub fn c(&self) -> [f64; 3] {
        self.m[2]
    }

    pub fn reduced(&self, a: f64, b: f64, c: f64) -> [f64; 3] {
        [
            a * self.m[0][0] + b * self.m[1][0] + c * self.m[2][0],
            a * self.m[0][1] + b * self.m[1][1] + c * self.m[2][1],
            a * self.m[0][2] + b * self.m[1][2] + c * self.m[2][2],
        ]
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
