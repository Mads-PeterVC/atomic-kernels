mod convert;
mod pyfunctions;
mod utils;
mod viewer_config;

pub use convert::{PyStructure, PyTrajectory};
use utils::create_structure;
pub use viewer_config::{PyColorConfig, PyLightingConfig, PyRenderConfig, PyViewerConfig};

use pyfunctions::py_distance_matrix::distance_matrix;
use pyfunctions::py_neighbor_list::neighborlist;
use pyfunctions::py_viewer::{
    PyPreparedHeadlessRender, PyPreparedViewerSession, PyViewerSession, launch_viewer,
    prepare_render_viewer_image, prepare_viewer_session, render_viewer_image, run_viewer_session,
    trajectory_viewer, viewer,
};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn _ak_viewer(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distance_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(neighborlist, m)?)?;
    m.add_function(wrap_pyfunction!(trajectory_viewer, m)?)?;
    m.add_function(wrap_pyfunction!(viewer, m)?)?;
    m.add_function(wrap_pyfunction!(launch_viewer, m)?)?;
    m.add_function(wrap_pyfunction!(run_viewer_session, m)?)?;
    m.add_function(wrap_pyfunction!(prepare_viewer_session, m)?)?;
    m.add_function(wrap_pyfunction!(render_viewer_image, m)?)?;
    m.add_function(wrap_pyfunction!(prepare_render_viewer_image, m)?)?;
    m.add_class::<PyViewerConfig>()?;
    m.add_class::<PyLightingConfig>()?;
    m.add_class::<PyColorConfig>()?;
    m.add_class::<PyRenderConfig>()?;
    m.add_class::<PyViewerSession>()?;
    m.add_class::<PyPreparedViewerSession>()?;
    m.add_class::<PyPreparedHeadlessRender>()?;
    Ok(())
}
