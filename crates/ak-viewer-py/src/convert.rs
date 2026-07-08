use ak_core::{Structure, Trajectory};
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

pub struct PyTrajectory(pub Trajectory);

impl Deref for PyTrajectory {
    type Target = Trajectory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Identifies the type of input object
pub(crate) enum InputType {
    /// ASE Atoms object
    Atoms,
    /// Tuple of arrays
    ArrayTuple,
}

/// Classify the input PyAny object
fn classify_input(obj: &Borrowed<'_, '_, PyAny>) -> PyResult<InputType> {
    // Check for ASE Atoms object (has positions and numbers attributes)
    if obj.hasattr("positions")? && obj.hasattr("numbers")? {
        return Ok(InputType::Atoms);
    }

    // Try to cast as tuple
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

impl FromPyObject<'_, '_> for PyTrajectory {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        // Try to extract as a sequence/list
        let seq = obj
            .cast::<pyo3::types::PySequence>()
            .map_err(|_| PyValueError::new_err("Expected a list/sequence of Atoms objects"))?;

        let length = seq.len()?;
        let mut structures = Vec::with_capacity(length);

        // Convert each item to a Structure using the existing infrastructure
        for i in 0..length {
            let item = seq.get_item(i)?;
            let borrowed = item.as_borrowed();

            // Classify and convert each item (could be Atoms object or tuple)
            let input_type = classify_input(&borrowed)?;
            let structure = match input_type {
                InputType::Atoms => structure_from_atoms(borrowed)?,
                InputType::ArrayTuple => structure_from_tuple(borrowed)?,
            };

            structures.push(structure);
        }

        Ok(PyTrajectory(Trajectory::new(structures)))
    }
}
