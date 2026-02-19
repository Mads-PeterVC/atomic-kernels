use crate::{PyStructure, PyTrajectory};
use ak_vis::{run_default, run_structure_default};
use pyo3::prelude::*;

#[pyfunction]
pub fn viewer(_py: Python<'_>, structure: PyStructure) {
    run_structure_default(structure.0);
}

#[pyfunction]
pub fn trajectory_viewer(_py: Python<'_>, trajectory: PyTrajectory) {
    run_default(trajectory.0);
}
