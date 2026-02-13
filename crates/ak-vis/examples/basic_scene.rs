use ak_core::Structure;
use ak_vis::viewer::run_structure_default;

fn main() {
    let structure = Structure::from_xyz_file(
        "/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/optimized_structure.xyz",
    );
    run_structure_default(structure);
}
