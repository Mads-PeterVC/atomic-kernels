use crate::geometry::{AtomicNumber, Cell, Pbc};

pub struct StructureView<'a> {
    pub positions: &'a [[f64; 3]],
    pub numbers: &'a [AtomicNumber],
    pub cell: Cell,
    pub pbc: Pbc,
}

impl<'a> StructureView<'a> {
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Structure;

    #[test]
    fn is_empty_is_true_for_empty_views() {
        let structure = Structure::new(Vec::new(), Vec::new(), [[0.0; 3]; 3], [false; 3]);

        assert!(structure.view().is_empty());
    }

    #[test]
    fn is_empty_is_false_for_populated_views() {
        let structure = Structure::new(
            vec![[0.0, 0.0, 0.0]],
            vec![1],
            [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
            [false; 3],
        );

        assert!(!structure.view().is_empty());
    }
}
