pub struct NeighborList {
    pub i: Vec<usize>,
    pub j: Vec<usize>,
    pub shifts: Vec<[i32; 3]>,
    pub distance: Option<Vec<f64>>,
}
