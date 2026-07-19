mod lennard_jones;
mod pair_potential;
mod structs;

pub use lennard_jones::LennardJones;
pub use pair_potential::PairPotential;
pub use structs::{Calculator, CalculatorError, CalculatorResult};

pub mod prelude {
    pub use super::LennardJones;
    pub use super::{Calculator, CalculatorError, CalculatorResult, PairPotential};
}
