use ak_core::Structure;
use ak_vis::viewer::{ColorConfig, RenderConfig, ViewerConfig, run_structure};
use bevy::color::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(
    positions: Vec<[f64; 3]>,
    numbers: Vec<i32>,
    cell: Vec<[Vec<[f64; 3]>; 3]>,
    pbc: Vec<[bool; 3]>,
) {
    let positions = vec![
        [5., 6.395248, 5.],
        [6.20832, 5.697624, 5.],
        [6.20832, 4.302376, 5.],
        [5., 3.604752, 5.],
        [3.79168, 4.302376, 5.],
        [3.79168, 5.697624, 5.],
        [5., 7.48236, 5.],
        [7.149787, 6.24118, 5.],
        [7.149787, 3.75882, 5.],
        [5., 2.51764, 5.],
        [2.850213, 3.75882, 5.],
        [2.850213, 6.24118, 5.],
    ];
    let numbers = vec![6, 6, 6, 6, 6, 6, 1, 1, 1, 1, 1, 1];
    let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
    let pbc = [false, false, false];

    let structure = Structure::new(positions, numbers, cell, pbc);

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
