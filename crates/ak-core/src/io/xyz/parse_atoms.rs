use std::str::SplitWhitespace;

use crate::PERIODIC_TABLE;
use crate::io::xyz::errors::AtomsParseError;

fn parse_coordinate(it: &mut SplitWhitespace<'_>, index: usize) -> Result<f64, AtomsParseError> {
    let coord: f64 = it
        .next()
        .ok_or(AtomsParseError::MissingCoordinate)?
        .parse()
        .map_err(|_| AtomsParseError::ParseF64Error { atom_index: index })?;

    Ok(coord)
}

pub(super) fn parse_atoms(
    lines: &[String],
    number_of_atoms: usize,
) -> Result<(Vec<[f64; 3]>, Vec<i32>), AtomsParseError> {
    let mut positions: Vec<[f64; 3]> = Vec::with_capacity(number_of_atoms);
    let mut numbers: Vec<i32> = Vec::with_capacity(number_of_atoms);
    for (index, line) in lines.iter().enumerate().take(number_of_atoms) {
        let mut it = line.split_whitespace();

        // Symbol parsing
        let symbol = it.next().ok_or(AtomsParseError::EmptyLine)?;
        let number = PERIODIC_TABLE
            .get_by_symbol_opt(symbol)
            .ok_or(AtomsParseError::SymbolError)?
            .number
            .get();
        numbers.push(number as i32);

        // Position parsing
        let x = parse_coordinate(&mut it, index)?;
        let y = parse_coordinate(&mut it, index)?;
        let z = parse_coordinate(&mut it, index)?;
        positions.push([x, y, z]);
    }

    Ok((positions, numbers))
}

#[cfg(test)]
mod tests {
    use super::{AtomsParseError, parse_atoms};

    #[test]
    fn test_parse_atoms() {
        let test_input = r#"P        5.00000000       4.69983800       5.37385700
H        5.00000000       5.90048500       4.62614300
H        6.03979100       4.09951500       4.62614300
H        3.96020900       4.09951500       4.62614300
"#
        .to_string();
        let lines = test_input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let (positions, numbers) = parse_atoms(&lines, 4).expect("Should parse");

        assert_eq!(numbers, vec![15, 1, 1, 1]);

        assert_eq!(
            positions,
            vec![
                [5.00000000, 4.69983800, 5.37385700],
                [5.00000000, 5.90048500, 4.62614300],
                [6.03979100, 4.09951500, 4.62614300],
                [3.96020900, 4.09951500, 4.62614300]
            ]
        )
    }

    #[test]
    fn rejects_invalid_coordinate() {
        let lines = vec!["P 5.0A000000 4.69983800 5.37385700".to_owned()];
        let result = parse_atoms(&lines, 1);

        assert!(matches!(
            result,
            Err(AtomsParseError::ParseF64Error { atom_index: 0 })
        ));
    }

    #[test]
    fn rejects_missing_coordinate() {
        let lines = vec!["P 5.00000000 5.37385700".to_owned()];
        let result = parse_atoms(&lines, 1);
        assert!(matches!(result, Err(AtomsParseError::MissingCoordinate)));
    }

    #[test]
    fn rejects_invalid_symbol() {
        let lines = vec!["Q 5.0000000 5.00000000 5.37385700".to_owned()];
        let result = parse_atoms(&lines, 1);
        assert!(matches!(result, Err(AtomsParseError::SymbolError)));
    }
}
