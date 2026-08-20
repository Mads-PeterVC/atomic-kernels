use crate::geometry::{AtomicNumber, Cell, Pbc, StructureView};

#[derive(Clone)]
pub struct Structure {
    pub positions: Vec<[f64; 3]>,
    pub numbers: Vec<AtomicNumber>,
    pub cell: Cell,
    pub pbc: Pbc,
}

impl Structure {
    pub fn new(
        positions: Vec<[f64; 3]>,
        numbers: Vec<i32>,
        cell: [[f64; 3]; 3],
        pbc: [bool; 3],
    ) -> Self {
        assert_eq!(positions.len(), numbers.len());

        let atomic_numbers = numbers
            .iter()
            .map(|z| AtomicNumber::new(*z).expect("Atomic number was not in [1, 110]"))
            .collect();

        Self {
            positions,
            numbers: atomic_numbers,
            cell: { Cell::new(cell) },
            pbc: { Pbc::new(pbc) },
        }
    }

    pub fn view(&self) -> StructureView<'_> {
        StructureView {
            positions: &self.positions,
            numbers: &self.numbers,
            cell: self.cell,
            pbc: self.pbc,
        }
    }

    pub fn from_xyz_reader<R: std::io::BufRead>(r: R) -> Self {
        crate::io::xyz::read_xyz_single(r).expect("XYZ file was not readable")
    }

    pub fn from_xyz_file<P: AsRef<std::path::Path>>(path: P) -> Self {
        let f = std::fs::File::open(path).unwrap();
        let r = std::io::BufReader::new(f);
        Self::from_xyz_reader(r)
    }
}

#[cfg(test)]
mod test {
    use crate::Structure;

    fn test_structure() -> Structure {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]].to_vec();
        let numbers = [1, 1].to_vec();
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        Structure::new(positions.clone(), numbers.clone(), cell, pbc)
    }

    #[test]
    fn test_positions() {
        let structure = test_structure();
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]].to_vec();
        assert_eq!(structure.positions, positions)
    }

    #[test]
    fn test_view() {
        let structure = test_structure();
        let view = structure.view();
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]].to_vec();
        assert_eq!(view.positions, positions)
    }
}
