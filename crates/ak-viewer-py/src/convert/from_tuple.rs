use ak_core::Structure;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use super::from_arrays::from_arrays;

/// Convert a tuple of 4 arrays to Structure
pub fn structure_from_tuple(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Structure> {
    let tuple = obj.cast::<pyo3::types::PyTuple>()?;

    if tuple.len() != 4 {
        return Err(PyValueError::new_err(format!(
            "Expected tuple of 4 arrays, got {} items",
            tuple.len()
        )));
    }

    let positions: numpy::PyReadonlyArray2<f64> = tuple.get_item(0)?.extract()?;
    let numbers: numpy::PyReadonlyArray1<i32> = tuple.get_item(1)?.extract()?;
    let cell: numpy::PyReadonlyArray2<f64> = tuple.get_item(2)?.extract()?;
    let pbc: numpy::PyReadonlyArray1<bool> = tuple.get_item(3)?.extract()?;

    from_arrays(&positions, &numbers, &cell, &pbc)
}
