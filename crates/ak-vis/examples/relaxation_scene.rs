use ak_core::{Structure, Trajectory};
use ak_vis::viewer::run_default;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // e.g., "/path/to/crates/ak-vis"
    let xyz_dir = Path::new(manifest_dir).join("examples/xyz/relaxation");

    // Read directory entries
    let mut structures: Vec<Structure> = Vec::new();

    // Read all entries in the directory
    let entries: Vec<fs::DirEntry> = fs::read_dir(xyz_dir).expect("Failed to read xyz directory").map(|f| f.expect("Stuff")).collect();

    for frame_index in 0..entries.len() {
        
        let path = format!("{manifest_dir}/examples/xyz/relaxation/frame_{frame_index:03}.xyz");

        let structure = Structure::from_xyz_file(&path);
        structures.push(structure);
        }

    let trajectory = Trajectory::new(structures);
    run_default(trajectory);
}
