use crate::PairPotential;

pub struct LennardJones {
    epsilon: f64,
    sigma: f64,
    cutoff: f64,
}

impl LennardJones {
    pub fn new(epsilon: f64, sigma: f64, cutoff: f64) -> Self {
        Self {
            epsilon,
            sigma,
            cutoff,
        }
    }
}

impl PairPotential for LennardJones {
    fn pair_energy(&self, r: f64) -> f64 {
        let x6 = (self.sigma / r).powi(6);
        let x12 = x6.powi(2);
        4.0 * self.epsilon * (x12 - x6)
    }

    fn pair_force_magnitude(&self, r: f64) -> f64 {
        let x6 = (self.sigma / r).powi(6);
        let x12 = x6.powi(2);
        24.0 * self.epsilon * (2.0 * x12 - x6) / r
    }

    fn cutoff(&self) -> f64 {
        self.cutoff
    }

    fn name(&self) -> &str {
        "Lennard Jones"
    }
}
