mod utils;
use utils::create_structure;

mod pyfunctions;
use pyfunctions::py_distance_matrix::distance_matrix;
use pyfunctions::py_neighbor_list::neighborlist;
use pyfunctions::py_viewer::viewer;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn _atomic_kernels(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distance_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(neighborlist, m)?)?;
    m.add_function(wrap_pyfunction!(viewer, m)?)?;
    Ok(())
}
