mod cell_list;
mod naive;
mod nl;

pub use cell_list::pair_traversal::cell_list_neighborlist;
pub use naive::main::naive_neighbor_list;
pub use nl::{NeighborList, NeighborListError};

use ak_core::StructureView;
use std::str::FromStr;

pub enum NeighborListMethod {
    Naive,
    CellList,
}

impl FromStr for NeighborListMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "cell-list" | "cell_list" => Ok(Self::CellList),
            other => Err(format!("Unknown method {other:?}")),
        }
    }
}

pub fn calculate_neighborlist(
    view: &StructureView,
    cutoff: f64,
    method: NeighborListMethod,
) -> Result<NeighborList, NeighborListError> {
    match method {
        NeighborListMethod::Naive => Ok(naive_neighbor_list(view, cutoff)),
        NeighborListMethod::CellList => cell_list_neighborlist(view, cutoff),
    }
}
