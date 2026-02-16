use ak_core::Structure;
use ak_vis::viewer::{run_structure, ColorConfig, ViewerConfig, RenderConfig};

use bevy::color::Color;

fn main() {
    let structure = Structure::from_xyz_file(
        "/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/optimized_structure.xyz",
    );

    let config = ViewerConfig {
        color: ColorConfig {
            background: Color::srgb(0.0, 0.0, 0.0),
            cell_color: Color::srgb(0.9, 0.0, 0.0),
        },
        render: RenderConfig {
            show_ui: true, 
            ..Default::default()
        },
        ..Default::default()
    };

    run_structure(structure, config);
}
