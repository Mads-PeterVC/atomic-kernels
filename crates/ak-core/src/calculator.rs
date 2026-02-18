pub mod lennard_jones;
pub mod pair_potential;
pub mod structs;

pub use structs::{Calculator, CalculatorError};

pub use pair_potential::PairPotential;

pub use lennard_jones::LennardJones;

pub mod prelude {
    pub use super::LennardJones;
    pub use super::{Calculator, CalculatorError, PairPotential};
}
