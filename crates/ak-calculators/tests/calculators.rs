use ak_calculators::{Calculator, LennardJones, PairPotential};
use ak_core::Structure;

fn test_structure_2atoms(sigma: f64) -> Structure {
    let rmin: f64 = 2.0_f64.powf(1.0 / 6.0) * sigma;

    let positions = [[0.0, 0.0, 0.0], [rmin, 0.0, 0.0]].to_vec();
    let numbers = [1, 1].to_vec();
    let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
    let pbc = [false, false, false];
    Structure::new(positions, numbers, cell, pbc)
}

fn test_structure_3atoms(sigma: f64) -> Structure {
    let rmin: f64 = 2.0_f64.powf(1.0 / 6.0) * sigma;

    let positions = [[0.0, 0.0, 0.0], [rmin, 0.0, 0.0], [2.0 * rmin, 0.0, 0.0]].to_vec();
    let numbers = [1, 1, 1].to_vec();
    let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
    let pbc = [false, false, false];
    Structure::new(positions, numbers, cell, pbc)
}

#[test]
fn pair_energy_test() {
    let lj = LennardJones::new(1.0, 1.0, 5.0);
    let rmin: f64 = 2.0_f64.powf(1.0 / 6.0);

    let pair_energy = lj.pair_energy(rmin);

    assert_eq!(pair_energy, -1.0);
}

#[test]
fn calculate_test_two_atoms() {
    let lj = LennardJones::new(1.5, 1.0, 5.0);
    let structure = test_structure_2atoms(1.0);
    let result = lj.calculate(&structure.view()).unwrap();
    assert_eq!(result.energy, Some(-1.5));
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
    let structure = test_structure_3atoms(1.0);
    let result = lj.calculate(&structure.view()).unwrap();
    assert_eq!(result.energy, Some(-2.5));
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
    let structure = test_structure_3atoms(1.0);
    let result = lj.calculate(&structure.view()).unwrap();
    assert!(result.energy.unwrap() < -2.5)
}

#[test]
fn lennard_jones_implements_pair_potential_and_calculator_contracts() {
    let sigma = 1.0;
    let epsilon = 2.0;
    let rmin = 2.0_f64.powf(1.0 / 6.0) * sigma;
    let structure = Structure::new(
        vec![[0.0, 0.0, 0.0], [rmin, 0.0, 0.0]],
        vec![1, 1],
        [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
        [false, false, false],
    );
    let potential = LennardJones::new(epsilon, sigma, 5.0);
    let result = potential.calculate(&structure.view()).unwrap();

    assert_eq!(potential.name(), "Lennard Jones");
    assert_eq!(potential.cutoff(), 5.0);
    assert_eq!(potential.pair_energy(rmin), -epsilon);
    assert_eq!(result.energy, Some(-epsilon));
    assert!(
        result
            .forces
            .unwrap()
            .into_iter()
            .flatten()
            .all(|component: f64| component.abs() < 1e-10)
    );
}
