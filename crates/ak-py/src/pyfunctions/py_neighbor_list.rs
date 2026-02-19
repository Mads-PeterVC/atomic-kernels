use crate::PyStructure;
use ak_core::build_neighborlist;
use numpy::ndarray::{Array1, Array2};
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

type NeighborListResult = PyResult<(Py<PyArray1<usize>>, Py<PyArray1<usize>>, Py<PyArray2<i32>>)>;

#[pyfunction]
pub fn neighborlist(
    py: Python<'_>,
    structure: PyStructure,
    cutoff: f64,
) -> NeighborListResult {
    // Thanks to Deref, we can call .view() directly instead of structure.0.view()
    let view = structure.view();

    // Call ak-core implementation
    let nl = build_neighborlist(&view, cutoff);

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
