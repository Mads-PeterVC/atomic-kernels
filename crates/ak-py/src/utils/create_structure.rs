use ak_core::Structure;
use numpy::PyUntypedArrayMethods;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub fn create_structure(
    positions: &numpy::PyReadonlyArray2<f64>,
    numbers: &numpy::PyReadonlyArray1<i32>,
    cell: &numpy::PyReadonlyArray2<f64>,
    pbc: &numpy::PyReadonlyArray1<bool>,
) -> PyResult<Structure> {
    let shape = positions.shape();
    if shape.len() != 2 || shape[1] != 3 {
        return Err(PyValueError::new_err(format!(
            "positions must have shape (N, 3); got ({}, {})",
            shape.get(0).copied().unwrap_or(0),
            shape.get(1).copied().unwrap_or(0),
        )));
    }

    let n_positions = shape[0];

    let pos_slice = positions.as_slice()?;
    let positions_vec: Vec<[f64; 3]> = pos_slice
        .chunks_exact(3)
        .map(|c| c.try_into().unwrap())
        .collect();

    let numbers_slice = numbers.as_slice()?;
    if numbers_slice.len() != n_positions {
        return Err(PyValueError::new_err(
            "numbers length must match positions length",
        ));
    }

    let cell_shape = cell.shape();
    if cell_shape != [3, 3] {
        return Err(PyValueError::new_err("cell must have shape (3, 3)"));
    }

    let cell_slice = cell.as_slice()?;
    let cell_vec: Vec<[f64; 3]> = cell_slice
        .chunks_exact(3)
        .map(|c| c.try_into().unwrap())
        .collect();

    let cell_arr: [[f64; 3]; 3] = cell_vec
        .try_into()
        .map_err(|_| PyValueError::new_err("cell must have shape (3, 3)"))?;

    let pbc_slice = pbc.as_slice()?;
    // let pbc_arr: Vec<[bool; 3]> = pbc_slice.chunks_exact(3).map(|c| c.try_into().unwrap()).collect().try_into().map_err(|_| PyValueError::new_err("PBC error"))?;
    let pbc_arr: [bool; 3] = pbc_slice.try_into()?;

    Ok(Structure::new(
        positions_vec,
        numbers_slice.to_vec(),
        cell_arr,
        pbc_arr,
    ))
}
