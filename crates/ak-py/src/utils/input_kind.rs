use pyo3::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputKind {
    Atoms,   // ASE-like object
    Dict,    // mapping with keys positions/numbers/cell/pbc
    Arrays,  // (positions, numbers, cell, pbc) passed as a tuple/sequence
}

/// Decide which input shape we got.
/// Return None if it matches none of the supported patterns.
pub fn check_input_type(ob: &Bound<'_, PyAny>) -> PyResult<Option<InputKind>> {
    // 1) Dict-like: try key access (fast & explicit)
    let looks_like_dict = ob.get_item("positions").is_ok()
        && ob.get_item("numbers").is_ok()
        && ob.get_item("cell").is_ok()
        && ob.get_item("pbc").is_ok();
    if looks_like_dict {
        return Ok(Some(InputKind::Dict));
    }

    // 2) ASE Atoms-like: duck-typing on common API
    let looks_like_atoms = ob.hasattr("get_positions")?
        && ob.hasattr("numbers")?
        && ob.hasattr("cell")?
        && ob.hasattr("pbc")?;
    if looks_like_atoms {
        return Ok(Some(InputKind::Atoms));
    }

    // 3) Arrays bundled as a 4-tuple / sequence (optional support)
    // Keep this last because lots of things are sequences in Python.
    match ob.cast::<pyo3::types::PySequence>() {
        Ok(seq) if seq.len()? == 4 => return Ok(Some(InputKind::Arrays)),
        _ => {}
    }

    Ok(None)
}