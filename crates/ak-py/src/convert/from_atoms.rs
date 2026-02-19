use ak_core::Structure;
use pyo3::prelude::*;

/// Convert an ASE Atoms object to Structure
pub fn structure_from_atoms(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Structure> {
    let positions: numpy::PyReadonlyArray2<f64> = obj.getattr("positions")?.extract()?;
    
    // ASE uses int64 for atomic numbers, need to convert to i32
    let numbers_i64: numpy::PyReadonlyArray1<i64> = obj.getattr("numbers")?.extract()?;
    let numbers_vec: Vec<i32> = numbers_i64
        .as_slice()?
        .iter()
        .map(|&n| n as i32)
        .collect();
    
    // ASE returns Cell object, need to get the underlying array
    let cell_obj = obj.getattr("cell")?;
    let cell: numpy::PyReadonlyArray2<f64> = if cell_obj.hasattr("array")? {
        // ase.cell.Cell has an .array attribute
        cell_obj.getattr("array")?.extract()?
    } else {
        // Fallback: try direct extraction
        cell_obj.extract()?
    };
    
    let pbc: numpy::PyReadonlyArray1<bool> = obj.getattr("pbc")?.extract()?;

    // Convert positions to Vec<[f64; 3]>
    let pos_slice = positions.as_slice()?;
    let positions_vec: Vec<[f64; 3]> = pos_slice
        .chunks_exact(3)
        .map(|c| c.try_into().unwrap())
        .collect();
    
    // Convert cell to [[f64; 3]; 3]
    let cell_slice = cell.as_slice()?;
    let cell_vec: Vec<[f64; 3]> = cell_slice
        .chunks_exact(3)
        .map(|c| c.try_into().unwrap())
        .collect();
    let cell_arr: [[f64; 3]; 3] = cell_vec
        .try_into()
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("cell must have shape (3, 3)"))?;
    
    // Convert pbc to [bool; 3]
    let pbc_slice = pbc.as_slice()?;
    let pbc_arr: [bool; 3] = pbc_slice.try_into()?;
    
    Ok(Structure::new(positions_vec, numbers_vec, cell_arr, pbc_arr))
}
