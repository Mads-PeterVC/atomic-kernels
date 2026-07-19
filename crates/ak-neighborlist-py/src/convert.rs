use ak_core::Structure;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::ops::Deref;

mod from_arrays;
mod from_atoms;
mod from_tuple;

pub use from_atoms::structure_from_atoms;
pub use from_tuple::structure_from_tuple;

pub struct PyStructure(pub Structure);

impl Deref for PyStructure {
    type Target = Structure;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) enum InputType {
    Atoms,
    ArrayTuple,
}

fn classify_input(obj: &Borrowed<'_, '_, PyAny>) -> PyResult<InputType> {
    if obj.hasattr("positions")? && obj.hasattr("numbers")? {
        return Ok(InputType::Atoms);
    }

    if obj.is_instance_of::<pyo3::types::PyTuple>() {
        return Ok(InputType::ArrayTuple);
    }

    Err(PyValueError::new_err(
        "Expected an ASE Atoms object or a tuple of 4 arrays",
    ))
}

impl FromPyObject<'_, '_> for PyStructure {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        let input_type = classify_input(&obj)?;

        let structure = match input_type {
            InputType::Atoms => structure_from_atoms(obj)?,
            InputType::ArrayTuple => structure_from_tuple(obj)?,
        };

        Ok(PyStructure(structure))
    }
}
