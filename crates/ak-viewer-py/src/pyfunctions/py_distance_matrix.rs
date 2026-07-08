use crate::create_structure;

use ak_core::distance_matrix as _distance_matrix;
use numpy::ndarray::Array2;
use numpy::{IntoPyArray, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn distance_matrix(
    py: Python<'_>,
    positions: numpy::PyReadonlyArray2<f64>,
    numbers: numpy::PyReadonlyArray1<i32>,
    cell: numpy::PyReadonlyArray2<f64>,
    pbc: numpy::PyReadonlyArray1<bool>,
) -> PyResult<Py<PyArray2<f64>>> {
    let structure = create_structure(&positions, &numbers, &cell, &pbc)?;
    let view = structure.view();

    // Call ak-core implementation
    let flat = _distance_matrix(&view);

    let n_positions = view.positions.len();
    let arr = Array2::from_shape_vec((n_positions, n_positions), flat)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(arr.into_pyarray(py).to_owned().into())
}
