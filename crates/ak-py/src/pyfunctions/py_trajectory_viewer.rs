use crate::PyTrajectory;
use ak_vis::run_default;
use pyo3::prelude::*;

#[pyfunction]
pub fn trajectory_viewer(_py: Python<'_>, trajectory: PyTrajectory) {
    run_default(trajectory.0);
}
