use crate::{
    PERIODIC_TABLE, Structure,
    io::xyz::errors::{CellError, XYZReaderError},
    io::xyz::parse_atoms::parse_atoms,
    io::xyz::parse_cell::parse_cell,
    io::xyz::parse_pbc::parse_pbc,
    io::xyz::xyz_main::XYZReaderState::{FindNumberOfAtoms, FindPositionLines, FindProperties},
};

#[derive(Debug)]
enum XYZReaderState {
    FindNumberOfAtoms,
    FindProperties,
    FindPositionLines(usize),
}

struct StructureBuffer {
    n_atoms: usize,
    properties: String,
    positions: Vec<String>,
}

impl StructureBuffer {
    fn new() -> StructureBuffer {
        StructureBuffer {
            n_atoms: 0,
            properties: String::new(),
            positions: Vec::new(),
        }
    }

    fn set_n_atoms(&mut self, n_atoms: usize) {
        self.n_atoms = n_atoms
    }

    fn get_n_atoms(&self) -> usize {
        self.n_atoms
    }

    fn set_properties(&mut self, properties: String) {
        self.properties = properties
    }

    fn push_position(&mut self, position: String) {
        self.positions.push(position)
    }

    fn positions(&self) -> &Vec<String> {
        &self.positions
    }

    fn properties(&self) -> &str {
        &self.properties
    }
}

pub fn read_xyz<R: std::io::BufRead>(r: R) -> Result<Vec<Structure>, XYZReaderError> {
    let mut structures: Vec<Structure> = Vec::new();
    let mut state = XYZReaderState::FindNumberOfAtoms;
    let mut structure_buffer = StructureBuffer::new();

    for line_result in r.lines() {
        let line = line_result.map_err(|_| XYZReaderError::LineIO)?;

        match state {
            FindNumberOfAtoms => {
                let n_atoms: usize = line.parse().map_err(|_| XYZReaderError::IntParse)?;
                structure_buffer.set_n_atoms(n_atoms);
                state = FindProperties;
            }
            FindProperties => {
                structure_buffer.set_properties(line);
                state = FindPositionLines(structure_buffer.n_atoms);
            }
            FindPositionLines(lines_left) => {
                state = if lines_left - 1 == 0 || lines_left == 0 {
                    structure_buffer.push_position(line);
                    let structure = buffer_to_structure(structure_buffer)?;
                    structures.push(structure);
                    structure_buffer = StructureBuffer::new();
                    FindNumberOfAtoms
                } else {
                    structure_buffer.push_position(line);
                    FindPositionLines(lines_left - 1)
                };
            }
        }
    }

    match state {
        FindNumberOfAtoms => Ok(structures),
        _ => Err(XYZReaderError::IncompleteInput),
    }
}

fn buffer_to_structure(buffer: StructureBuffer) -> Result<Structure, XYZReaderError> {
    if !(buffer.get_n_atoms() == buffer.positions().len()) {
        return Err(XYZReaderError::IncorrectNumberOfAtoms);
    }

    let (positions, numbers) = parse_atoms(&buffer.positions(), buffer.get_n_atoms())?;
    let cell = parse_cell(&buffer.properties());

    let cell = match cell {
        Ok(cell_result) => cell_result,
        Err(cell_error) => match cell_error {
            CellError::NoCellSpecified => [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            _ => return Err(XYZReaderError::CellError(cell_error)),
        },
    };

    let pbc = parse_pbc(&buffer.properties())?;
    Ok(Structure::new(positions, numbers, cell, pbc))
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::read_xyz;

    #[test]
    fn simple_test() {
        let xyz = r#"4
Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
P        5.00000000       4.69983800       5.37385700
H        5.00000000       5.90048500       4.62614300
H        6.03979100       4.09951500       4.62614300
H        3.96020900       4.09951500       4.62614300
2
Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
P        5.00000000       5.00000000       5.96614400
P        5.00000000       5.00000000       4.03385600
7
Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
O        6.40718800       5.44571050       5.00000000
C        5.18913300       5.54860350       5.00000000
H        4.71189200       6.54976550       5.00000000
C        4.24103100       4.38433250       5.00000000
H        4.80318700       3.45023450       5.00000000
H        3.59281200       4.43199550       5.88094600
H        3.59281200       4.43199550       4.11905400
"#;
        let mut reader = BufReader::new(Cursor::new(xyz));
        let structures = read_xyz(&mut reader).unwrap();

        assert_eq!(structures.len(), 3);

        let n_atoms = structures
            .iter()
            .map(|x| x.numbers.len())
            .collect::<Vec<usize>>();

        assert_eq!(n_atoms, vec![4, 2, 7])
    }
}
