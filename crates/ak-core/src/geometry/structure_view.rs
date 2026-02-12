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
}
