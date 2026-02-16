use ak_core::Structure;
use ak_vis::viewer::{ColorConfig, RenderConfig, ViewerConfig, run_structure};

use bevy::color::Color;

fn main() {
    let structure = Structure::from_xyz_file(
        "/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/CH3CH2OCH3.xyz",
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
