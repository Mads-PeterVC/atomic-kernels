use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct Pbc(pub [bool; 3]);

impl Pbc {
    pub fn new(pbc: [bool; 3]) -> Self {
        Self(pbc)
    }

    pub fn any(self) -> bool {
        self.0.contains(&true)
    }

    pub fn all(self) -> bool {
        self.0.iter().all(|f| *f)
    }
}

impl Deref for Pbc {
    type Target = [bool; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::pbc::Pbc;

    #[test]
    fn test_pbc() {
        let pbc = Pbc::new([true, true, true]);
        assert!(pbc.0[0]);
        assert!(pbc.0[1]);
        assert!(pbc.0[2]);
    }

    #[test]
    fn test_pbc_deref() {
        let pbc = Pbc::new([true, true, true]);
        assert!(pbc[0]);
        assert!(pbc[1]);
        assert!(pbc[2]);
    }

    #[test]
    fn test_pbc_any() {
        let pbc = Pbc::new([true, false, false]);
        assert!(pbc.any());
    }

    #[test]
    fn test_pbc_any_false() {
        let pbc = Pbc::new([false, false, false]);
        assert!(!pbc.any());
    }

    #[test]
    fn test_pbc_all() {
        let pbc = Pbc::new([true, true, true]);
        assert!(pbc.all());
    }

    #[test]
    fn test_pbc_all_false() {
        let pbc = Pbc::new([true, true, false]);
        assert!(!pbc.all());
    }
}
