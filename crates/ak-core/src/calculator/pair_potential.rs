use std::iter::zip;

use crate::calculator::{Calculator, CalculatorError};
use crate::geometry::build_neighborlist;

pub trait PairPotential {
    fn cutoff(&self) -> f64;
    fn pair_energy(&self, r: f64) -> f64;
    fn pair_force_magnitude(&self, r: f64) -> f64;
    fn name(&self) -> &str;
}

impl<T: PairPotential> Calculator for T {
    fn calculate_energy(&self, view: &crate::StructureView) -> Result<f64, CalculatorError> {
        let nl = build_neighborlist(&view, self.cutoff());

        let mut energy = 0.0;
        let distances = nl.distance.ok_or(CalculatorError::CalculationFailed)?;

        for r in distances {
            energy += self.pair_energy(r);
        }
        Ok(energy)
    }

    fn calculate_forces(
        &self,
        view: &crate::StructureView,
    ) -> Result<Vec<[f64; 3]>, CalculatorError> {
        let nl = build_neighborlist(&view, self.cutoff());
        let distances = nl.distance.ok_or(CalculatorError::CalculationFailed)?;

        let mut forces: Vec<[f64; 3]> = vec![[0.0 as f64; 3]; view.len()];

        for ((i, j), r) in zip(zip(nl.i, nl.j), distances) {
            let pos_i = view.positions[i];
            let pos_j = view.positions[j];
            let r_vec = [
                pos_j[0] - pos_i[0],
                pos_j[1] - pos_i[1],
                pos_j[2] - pos_i[2],
            ];
            let f_mag = self.pair_force_magnitude(r);
            let r_hat = [r_vec[0] / r, r_vec[1] / r, r_vec[2] / r]; // unit vector

            forces[i] = [
                forces[i][0] + f_mag * r_hat[0],
                forces[i][1] + f_mag * r_hat[1],
                forces[i][2] + f_mag * r_hat[2],
            ];
            forces[j] = [
                forces[j][0] - f_mag * r_hat[0],
                forces[j][1] - f_mag * r_hat[1],
                forces[j][2] - f_mag * r_hat[2],
            ];
        }

        Ok(forces)
    }
}
