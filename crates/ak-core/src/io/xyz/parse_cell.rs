use super::errors::CellError;

pub(super) fn parse_cell(comment_line: &str) -> Result<[[f64; 3]; 3], CellError> {
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

#[cfg(test)]
mod test {
    use super::{CellError, parse_cell};

    #[test]
    fn empty_cell() {
        let line = r#"Properties=species:S:1:pos:R:3 pbc="F F F"#;
        let cell_result = parse_cell(line);
        assert!(matches!(cell_result, Err(CellError::NoCellSpecified)));
    }
    #[test]
    fn invalid_cell_f64() {
        let line = r#"Lattice="10.0 0.0 A.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
"#;
        let cell_result = parse_cell(line);
        assert!(matches!(cell_result, Err(CellError::ParseError)));
    }

    #[test]
    fn invalid_cell_not9() {
        let line = r#"Lattice="10.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
"#;
        let cell_result = parse_cell(line);
        assert!(matches!(cell_result, Err(CellError::Expected9Floats)));
    }

    #[test]
    fn test_cell_parse() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
"#;
        let lattice = parse_cell(line).unwrap();
        assert_eq!(
            lattice,
            [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]]
        );
    }
}
