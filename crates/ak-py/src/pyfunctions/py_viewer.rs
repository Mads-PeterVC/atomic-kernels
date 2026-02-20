use crate::{PyStructure, PyTrajectory, PyViewerConfig};
use ak_vis::{run, run_default, run_structure, run_structure_default};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (structure, config=None))]
pub fn viewer(_py: Python<'_>, structure: PyStructure, config: Option<PyViewerConfig>) {
    match config {
        Some(cfg) => run_structure(structure.0, cfg.inner.clone()),
        None => run_structure_default(structure.0),
    }
}

#[pyfunction]
#[pyo3(signature = (trajectory, config=None))]
pub fn trajectory_viewer(_py: Python<'_>, trajectory: PyTrajectory, config: Option<PyViewerConfig>) {
    match config {
        Some(cfg) => run(trajectory.0, cfg.inner.clone()),
        None => run_default(trajectory.0),
    }
}
