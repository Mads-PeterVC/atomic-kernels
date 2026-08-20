use super::errors::PBCError;

pub(super) fn parse_pbc(comment_line: &str) -> Result<[bool; 3], PBCError> {
    let pbc_start: usize = comment_line
        .match_indices("pbc=")
        .next()
        .map(|(idx, _)| idx)
        .ok_or(PBCError::NoPBCSpecified)?;

    let value = &comment_line[(pbc_start + "pbc=".len())..];

    let value = value
        .strip_prefix('"')
        .ok_or(PBCError::IncorrectFormatting)?;

    let closing_quote = value.find('"').ok_or(PBCError::IncorrectFormatting)?;

    let pbc_slice = &value[..closing_quote];

    let pbc_vec = pbc_slice
        .split_whitespace()
        .map(|flag| match flag {
            "f" | "F" | "false" | "FALSE" | "False" => Ok(false),
            "t" | "T" | "true" | "TRUE" | "True" => Ok(true),
            _ => Err(PBCError::UnknownCharacter),
        })
        .collect::<Result<Vec<bool>, PBCError>>()?;
    let pbc: [bool; 3] = pbc_vec.try_into().map_err(|_| PBCError::Expected3Flags)?;
    Ok(pbc)
}

#[cfg(test)]
mod test {

    use crate::io::xyz::errors::PBCError;

    use super::parse_pbc;

    #[test]
    fn test_parse_pbc_fff() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="F F F"
"#;
        let pbc = parse_pbc(line).unwrap();
        assert_eq!(pbc, [false, false, false]);
    }

    #[test]
    fn test_parse_pbc_ttt() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="T T T"
"#;
        let pbc = parse_pbc(line).unwrap();
        assert_eq!(pbc, [true, true, true]);
    }

    #[test]
    fn test_parse_pbc_xxx() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="x x x"
"#;
        let pbc_result = parse_pbc(line);
        assert!(matches!(pbc_result, Err(PBCError::UnknownCharacter)));
    }

    #[test]
    fn test_parse_pbc_tt() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="t t"
"#;
        let pbc_result = parse_pbc(line);
        assert!(matches!(pbc_result, Err(PBCError::Expected3Flags)));
    }

    #[test]
    fn test_parse_pbc_no_pbc() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pb="t t"
"#;
        let pbc_result = parse_pbc(line);
        assert!(matches!(pbc_result, Err(PBCError::NoPBCSpecified)));
    }

    #[test]
    fn test_parse_pbc_quote_no_start() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc=t t f"
"#;
        let pbc_result = parse_pbc(line);
        assert!(matches!(pbc_result, Err(PBCError::IncorrectFormatting)));
    }

    #[test]
    fn test_parse_pbc_quote_no_end() {
        let line = r#"Lattice="10.0 0.0 0.0 0.0 10.0 0.0 0.0 0.0 10.0" Properties=species:S:1:pos:R:3 pbc="t t f
"#;
        let pbc_result = parse_pbc(line);
        assert!(matches!(pbc_result, Err(PBCError::IncorrectFormatting)));
    }
}
