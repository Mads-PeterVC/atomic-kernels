use super::naive_neighbor_list::naive_neighbor_list_non_pbc;
use super::naive_neighbor_list_pbc::naive_neighbor_list_pbc;

use crate::NeighborList;

use ak_core::StructureView;

pub fn naive_neighbor_list(view: &StructureView, cutoff: f64) -> NeighborList {
    if view.pbc.any() {
        naive_neighbor_list_pbc(view, cutoff)
    } else {
        naive_neighbor_list_non_pbc(view, cutoff)
    }
}
