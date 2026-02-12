#[derive(Clone, Copy, Debug)]
pub struct Pbc(pub [bool; 3]);

impl Pbc {
    pub fn new(pbc: [bool; 3]) -> Self {
        Self(pbc)
    }
}
