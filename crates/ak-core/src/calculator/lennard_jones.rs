use crate::calculator::pair_potential::PairPotential;

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

#[cfg(test)]
mod test {
    use crate::Structure;
    use crate::calculator::prelude::*;

    fn test_structure_2atoms(sigma: f64) -> Structure {
        let rmin: f64 = 2.0_f64.powf(1.0 / 6.0) * sigma;

        let positions = [[0.0, 0.0, 0.0], [rmin, 0.0, 0.0]].to_vec();
        let numbers = [1, 1].to_vec();
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        Structure::new(positions.clone(), numbers.clone(), cell, pbc)
    }

    fn test_structure_3atoms(sigma: f64) -> Structure {
        let rmin: f64 = 2.0_f64.powf(1.0 / 6.0) * sigma;

        let positions = [[0.0, 0.0, 0.0], [rmin, 0.0, 0.0], [2.0 * rmin, 0.0, 0.0]].to_vec();
        let numbers = [1, 1, 1].to_vec();
        let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
        let pbc = [false, false, false];
        Structure::new(positions.clone(), numbers.clone(), cell, pbc)
    }

    #[test]
    fn pair_energy_test() {
        let lj = LennardJones::new(1.0, 1.0, 5.0);
        let rmin: f64 = 2.0_f64.powf(1.0 / 6.0) * lj.sigma;

        let pair_energy = lj.pair_energy(rmin);

        assert_eq!(pair_energy, -1.0);
    }

    #[test]
    fn calculate_test_two_atoms() {
        let lj = LennardJones::new(1.5, 1.0, 5.0);
        let structure = test_structure_2atoms(lj.sigma);
        let result = lj.calculate(&structure.view()).unwrap();
        assert_eq!(result.energy, Some(-lj.epsilon));
        assert!(
            result
                .forces
                .unwrap()
                .iter()
                .flatten()
                .all(|&f| f.abs() < 1e-10)
        );
    }

    #[test]
    fn calculate_test_three_atoms() {
        let lj = LennardJones::new(1.25, 1.0, 2.0);
        let structure = test_structure_3atoms(lj.sigma);
        let result = lj.calculate(&structure.view()).unwrap();
        assert_eq!(result.energy, Some(-2.0 * lj.epsilon));
        assert!(
            result
                .forces
                .unwrap()
                .iter()
                .flatten()
                .all(|&f| f.abs() < 1e-10)
        );
    }

    #[test]
    fn calculate_test_long_cutoff_three_atoms() {
        let lj = LennardJones::new(1.25, 1.0, 5.0);
        let structure = test_structure_3atoms(lj.sigma);
        let result = lj.calculate(&structure.view()).unwrap();
        assert!(result.energy.unwrap() < -2.0 * lj.epsilon)
    }
}
