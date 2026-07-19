mod convert;

pub use convert::PyStructure;

use ak_neighborlist::{naive_neighbor_list, naive_neighbor_list_pbc};
use numpy::ndarray::{Array1, Array2};
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

type NeighborListResult = PyResult<(Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray2<i32>>)>;

#[pyfunction]
pub fn neighborlist(py: Python<'_>, structure: PyStructure, cutoff: f64) -> NeighborListResult {
    let view = structure.view();

    let nl = if view.pbc.any() {
        naive_neighbor_list_pbc(&view, cutoff)
    } else {
        naive_neighbor_list(&view, cutoff)
    };

    let size = nl.i.len();

    let index_i_arr = Array1::from_vec(nl.i);
    let index_j_arr = Array1::from_vec(nl.j);

    let shifts_flat: Vec<i32> = nl.shifts.into_iter().flat_map(|v| v.into_iter()).collect();

    let shifts_arr = Array2::from_shape_vec((size, 3), shifts_flat)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok((
        index_i_arr.into_pyarray(py).unbind(),
        index_j_arr.into_pyarray(py).unbind(),
        shifts_arr.into_pyarray(py).unbind(),
    ))
}

#[pymodule]
fn _ak_neighborlist(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(neighborlist, m)?)?;
    Ok(())
}
