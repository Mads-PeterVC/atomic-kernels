#[derive(Debug, thiserror::Error)]
pub enum AtomsParseError {
    #[error("Failed to parse f64")]
    ParseF64Error { atom_index: usize },
    #[error("Coordinate missing")]
    MissingCoordinate,
    #[error("Empty line")]
    EmptyLine,
    #[error("Could not parse atomic symbol")]
    SymbolError,
}

#[derive(Debug, thiserror::Error)]
pub enum CellError {
    #[error("The file does not contain a cell")]
    NoCellSpecified,
    #[error("Expected 9 floats for cell specification")]
    Expected9Floats,
    #[error("Failed to construct cell")]
    ParseError,
    #[error("Formatting, such as missing quotes")]
    IncorrectFormatting,
}

#[derive(Debug, thiserror::Error)]
pub enum PBCError {
    #[error("PBC is not given")]
    NoPBCSpecified,
    #[error("Incorrect number of PBC flags")]
    Expected3Flags,
    #[error("Unknown character specified for PBC")]
    UnknownCharacter,
    #[error("The PBC specification is not quoted correctly")]
    IncorrectFormatting,
}

#[derive(Debug, thiserror::Error)]
pub enum XYZReaderError {
    #[error("Failed to read line")]
    LineIO,
    #[error("Failed to parse number of atoms")]
    IntParse,
    #[error("Input file is incomplete")]
    IncompleteInput,
    #[error("Failed to parse atoms data")]
    AtomsError(#[from] AtomsParseError),
    #[error("Failed to parse cell data")]
    CellError(#[from] CellError),
    #[error("Failed to parse pbc data")]
    PBCError(#[from] PBCError),
    #[error("Mismatched atoms & number of atoms")]
    IncorrectNumberOfAtoms,
    #[error("Empty file")]
    EmptyFile,
}
