use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NeighborListError {
    #[error("Stupid catchall error for now")]
    CatchAllError,
}

pub struct NeighborList {
    pub i: Vec<usize>,
    pub j: Vec<usize>,
    pub shifts: Vec<[i32; 3]>,
    pub distance: Option<Vec<f64>>,
}
