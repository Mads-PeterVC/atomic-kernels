use ak_core::Structure;
use ak_vis::viewer::run_structure_default;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let structure = Structure::from_xyz_file(
        format!("{manifest_dir}/examples/xyz/CH3CH2OCH3.xyz")
    );
    run_structure_default(structure);
}
