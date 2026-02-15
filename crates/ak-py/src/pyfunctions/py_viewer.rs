use crate::create_structure;
use ak_vis::run_structure_default;
use pyo3::prelude::*;

#[pyfunction]
pub fn viewer(
    _py: Python<'_>,
    positions: numpy::PyReadonlyArray2<f64>,
    numbers: numpy::PyReadonlyArray1<i32>,
    cell: numpy::PyReadonlyArray2<f64>,
    pbc: numpy::PyReadonlyArray1<bool>,
) {
    let structure = create_structure(&positions, &numbers, &cell, &pbc).expect("Failed to load structure");
    run_structure_default(structure);
}
