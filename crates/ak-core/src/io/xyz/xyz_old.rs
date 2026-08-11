use crate::{PERIODIC_TABLE, Structure};

#[derive(Debug)]
enum PositionParseError {
    ParseF64Error,
    MissingCoordinate,
    SymbolError,
}
#[derive(Debug)]

enum CellError {
    NoCellSpecified,
    Expected9Floats,
    ParseError,
}

#[derive(Debug)]
enum PBCError {
    NoPBCSpecified,
    Expected3Flags,
}

fn read_positions_and_numbers(
    lines: &[String],
    number_of_atoms: usize,
) -> Result<(Vec<[f64; 3]>, Vec<i32>), PositionParseError> {
    let mut positions: Vec<[f64; 3]> = Vec::with_capacity(number_of_atoms);
    let mut numbers: Vec<i32> = Vec::with_capacity(number_of_atoms);
    for line in lines.iter().take(number_of_atoms) {
        let mut it = line.split_whitespace();
        let symbol = it.next().ok_or(PositionParseError::SymbolError)?;
        let x: f64 = it
            .next()
            .ok_or(PositionParseError::MissingCoordinate)?
            .parse()
            .map_err(|_| PositionParseError::ParseF64Error)?;
        let y: f64 = it
            .next()
            .ok_or(PositionParseError::MissingCoordinate)?
            .parse()
            .map_err(|_| PositionParseError::ParseF64Error)?;
        let z: f64 = it
            .next()
            .ok_or(PositionParseError::MissingCoordinate)?
            .parse()
            .map_err(|_| PositionParseError::ParseF64Error)?;
        let position = [x, y, z];
        positions.push(position);

        let number = PERIODIC_TABLE.get_by_symbol(symbol).number.get();
        numbers.push(number as i32);
    }

    Ok((positions, numbers))
}

fn read_cell(comment_line: &str) -> Result<[[f64; 3]; 3], CellError> {
    let lattice_start: usize = comment_line
        .match_indices("Lattice=")
        .next()
        .map(|(idx, _)| idx)
        .ok_or(CellError::NoCellSpecified)?;

    let tail = &comment_line[lattice_start..];
    let quotes: Vec<usize> = tail.match_indices('"').map(|(i, _)| i).collect();
    let part = &tail[(quotes[0] + 1)..quotes[1]];
    let parts: Vec<f64> = part
        .split_whitespace()
        .map(|c| c.parse::<f64>().map_err(|_| CellError::ParseError))
        .collect::<Result<Vec<f64>, CellError>>()?;
    let arr: [f64; 9] = parts.try_into().map_err(|_| CellError::Expected9Floats)?;

    let cell: [[f64; 3]; 3] = [
        [arr[0], arr[1], arr[2]],
        [arr[3], arr[4], arr[5]],
        [arr[6], arr[7], arr[8]],
    ];
    Ok(cell)
}

fn read_pbc(comment_line: &str) -> Result<[bool; 3], PBCError> {
    let pbc_start: usize = comment_line
        .match_indices("pbc=")
        .next()
        .map(|(idx, _)| idx)
        .ok_or(PBCError::NoPBCSpecified)?;

    let tail = &comment_line[pbc_start..];
    let quotes: Vec<usize> = tail.match_indices('"').map(|(i, _)| i).collect();
    let pbc_slice = &tail[(quotes[0] + 1)..quotes[1]];
    let pbc_vec: Vec<bool> = pbc_slice
        .split_whitespace()
        .map(|flag| !matches!(flag, "f" | "F" | "false" | "FALSE" | "False"))
        .collect();
    let pbc: [bool; 3] = pbc_vec.try_into().map_err(|_| PBCError::Expected3Flags)?;
    Ok(pbc)
}

pub fn read_xyz<R: std::io::BufRead>(r: R) -> Structure {
    let mut lines = Vec::new();
    for line_result in r.lines() {
        let line = line_result.expect("Read failure");
        lines.push(line);
    }

    let number_of_atoms: usize = lines[0]
        .parse()
        .expect("Failed to convert atom number to int.");

    let (positions, numbers) =
        read_positions_and_numbers(&lines[2..], number_of_atoms).expect("Error");

    let cell =
        { read_cell(&lines[1]).unwrap_or([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]) };
    let pbc = { read_pbc(&lines[1]).unwrap_or_default() };

    Structure::new(positions, numbers, cell, pbc)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::io::read_xyz;

    #[test]
    fn simple_test() {
        let xyz = r#"3
Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
O        5.00000000       5.00000000       5.29815450
H        5.00000000       5.76323900       4.70184550
H        5.00000000       4.23676100       4.70184550
"#;
        let reader = BufReader::new(Cursor::new(xyz));
        let _s = read_xyz(reader);
    }
}
