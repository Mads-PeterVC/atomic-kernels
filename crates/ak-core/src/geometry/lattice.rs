use crate::Cell;
use std::f64::consts::PI;
use std::iter::zip;

#[derive(PartialEq, Debug)]
pub enum CellShape {
    Cubic,
    Orthorhombic,
    Triclinic,
    Monoclinic,
    Tetragonal,
    Hexagonal,
    Trigonal,
    Unmatched,
}

fn compare_within_tolerance(value: &f64, comparate: &f64, tol: &f64) -> bool {
    (value - comparate).abs() < *tol
}

impl Cell {
    pub fn lattice(&self) -> CellShape {
        let a = self.a();
        let b = self.b();
        let c = self.c();

        // Lattice vector lengths
        let a_len = (a[0].powf(2.0) + a[1].powf(2.0) + a[2].powf(2.0)).sqrt();
        let b_len: f64 = (b[0].powf(2.0) + b[1].powf(2.0) + b[2].powf(2.0)).sqrt();
        let c_len = (c[0].powf(2.0) + c[1].powf(2.0) + c[2].powf(2.0)).sqrt();

        let max_len = [a_len, b_len, c_len]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let a_len = a_len / max_len;
        let b_len = b_len / max_len;
        let c_len = c_len / max_len;

        // Lattice vector angles
        let alpha = b.angle(&c);
        let beta = a.angle(&c);
        let gamma = a.angle(&b);

        // Bools
        let eps_length = 1e-12;
        let mut equals_lengths: i32 = 0;

        for (v1, v2) in zip([a_len, a_len, b_len].iter(), [b_len, c_len, c_len].iter()) {
            if compare_within_tolerance(&v1, &v2, &eps_length) {
                equals_lengths += 1;
            }
        }

        let eps_angle = 1e-9;
        let mut equal_angles: i32 = 0;
        for (ang1, ang2) in zip([alpha, alpha, beta].iter(), [beta, gamma, gamma].iter()) {
            if compare_within_tolerance(&ang1, &ang2, &eps_angle) {
                equal_angles += 1
            }
        }

        let deg_90 = PI / 2.0;
        let deg_120 = 2.0 / 3.0 * PI;
        let mut count_90 = 0;
        let mut count_120: i32 = 0;

        for angle in [alpha, beta, gamma] {
            if compare_within_tolerance(&angle, &deg_90, &eps_angle) {
                count_90 += 1;
            }
            if compare_within_tolerance(&angle, &deg_120, &eps_angle)
                || compare_within_tolerance(&angle, &(deg_120 / 2.0), &eps_angle)
            {
                count_120 += 1;
            }
        }

        if count_90 == 0 && equals_lengths == 0 && equal_angles == 0 {
            CellShape::Triclinic
        } else if count_90 == 0 && equals_lengths == 3 && equal_angles == 3 {
            CellShape::Trigonal
        } else if count_90 == 2 && equals_lengths == 0 {
            CellShape::Monoclinic
        } else if count_90 == 2 && count_120 == 1 && equals_lengths == 1 {
            CellShape::Hexagonal
        } else if count_90 == 3 && equals_lengths == 0 {
            CellShape::Orthorhombic
        } else if count_90 == 3 && equals_lengths == 1 {
            CellShape::Tetragonal
        } else if count_90 == 3 && equals_lengths == 3 {
            CellShape::Cubic
        } else {
            CellShape::Unmatched
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CellShape;
    use crate::Cell;

    #[test]
    fn test_cubic() {
        let cell_f64 = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Cubic);
    }

    #[test]
    fn test_tetragonal() {
        let cell_f64 = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 12.0]];
        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Tetragonal);
    }

    #[test]
    fn test_orthorhombic() {
        let cell_f64 = [[10.0, 0.0, 0.0], [0.0, 11.0, 0.0], [0.0, 0.0, 12.0]];
        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Orthorhombic);
    }

    #[test]
    fn test_hexagonal() {
        let a = 10.0;
        let cell_f64 = [
            [a, 0.0, 0.0],
            [-0.5 * a, f64::sqrt(3.0) / 2.0 * a, 0.0],
            [0.0, 0.0, 12.0],
        ];
        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Hexagonal);
    }

    #[test]
    fn test_monoclinic() {
        let beta = std::f64::consts::PI * 0.75;
        let cell_f64 = [
            [1.1, 0.0, 0.0],
            [0.0, 1.2, 0.0],
            [f64::cos(beta), 0.0, f64::sin(beta)],
        ];
        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Monoclinic);
    }

    #[test]
    fn test_trigonal() {
        let alpha = std::f64::consts::PI * (55.0 / 180.0);
        let a = 4.5;

        let y3 = a * (alpha.cos() - alpha.cos().powf(2.0)) / alpha.sin();
        let z3 = a
            * (1.0
                - alpha.cos().powf(2.0)
                - ((alpha.cos() - alpha.cos().powf(2.0)) / alpha.sin()).powf(2.0))
            .sqrt();

        let cell_f64 = [
            [a, 0.0, 0.0],
            [a * alpha.cos(), a * alpha.sin(), 0.0],
            [a * alpha.cos(), y3, z3],
        ];

        let cell = Cell::new(cell_f64);
        assert_eq!(cell.lattice(), CellShape::Trigonal);
    }
}
