use ak_core::{Structure, Trajectory};
use ak_vis::viewer::run_default;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // e.g., "/path/to/crates/ak-vis"
    let xyz_dir = Path::new(manifest_dir).join("examples/xyz");

    // Read directory entries
    let mut structures: Vec<Structure> = Vec::new();

    // Read all entries in the directory
    let entries = fs::read_dir(xyz_dir).expect("Failed to read xyz directory");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        // Filter for .xyz files
        if path.extension().and_then(|s| s.to_str()) == Some("xyz") {
            let structure = Structure::from_xyz_file(&path);
            structures.push(structure);
        }
    }

    let trajectory = Trajectory::new(structures);
    run_default(trajectory);
}
