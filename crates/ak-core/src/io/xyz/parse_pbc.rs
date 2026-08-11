use super::errors::PBCError;

pub(super) fn parse_pbc(comment_line: &str) -> Result<[bool; 3], PBCError> {
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
