use ::bevy::prelude::*;
use ak_core::{Structure, Trajectory};
use ak_vis::viewer::app::build_app;
use ak_vis::viewer::session::ViewerSnapshot;
use ak_vis::viewer::session::{ViewerCommand, ViewerReadiness, ViewerSessionHandle};
use ak_vis::viewer::{ColorConfig, RenderConfig, ViewerConfig, run_structure};
use bevy::app::App;
use bevy::color::Color;
use std::sync::{Arc, Mutex, mpsc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmViewer {
    app: App,
    sender: std::sync::mpsc::Sender<ViewerCommand>,
}

#[wasm_bindgen]
impl WasmViewer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmViewer {
        let config = ViewerConfig {
            render: RenderConfig {
                show_ui: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let trajectory = Trajectory::new(Vec::new());
        let (sender, receiver) = std::sync::mpsc::channel();

        let app = build_app(
            trajectory,
            config,
            Some(receiver),
            Arc::new(ViewerReadiness::new()),
            Arc::new(Mutex::new(ViewerSnapshot::default())),
        );

        WasmViewer {
            app: app,
            sender: sender,
        }
    }

    pub fn run(&mut self) -> Result<(), JsValue> {
        self.app.run();
        Ok(())
    }

    pub fn append_frame(
        &self,
        positions_xyz: Vec<f64>, // flat: [x0,y0,z0,x1,y1,z1,...]
        numbers: Vec<i32>,
        cell_3x3: Vec<f64>, // flat length 9
    ) -> Result<(), JsValue> {
        let structure = structure_from_flat_arrays(positions_xyz, numbers, cell_3x3)?;

        self.sender
            .send(ViewerCommand::AppendFrame { frame: structure })
            .map_err(|_| JsValue::from_str("viewer is closed"))?;

        Ok(())
    }
}

fn structure_from_flat_arrays(
    positions_xyz: Vec<f64>, // flat: [x0,y0,z0,x1,y1,z1,...]
    numbers: Vec<i32>,
    cell_3x3: Vec<f64>, // flat length 9)
) -> Result<Structure, JsValue> {
    if positions_xyz.len() % 3 != 0 {
        return Err("positions length must be divisible by 3".into());
    }
    if cell_3x3.len() != 9 {
        return Err("bad cell or pbc length".into());
    }

    let positions: Vec<[f64; 3]> = positions_xyz
        .chunks_exact(3)
        .map(|c| [c[0], c[1], c[2]])
        .collect();

    let cell = [
        [cell_3x3[0], cell_3x3[1], cell_3x3[2]],
        [cell_3x3[3], cell_3x3[4], cell_3x3[5]],
        [cell_3x3[6], cell_3x3[7], cell_3x3[8]],
    ];

    let pbc = [false, false, false];

    Ok(Structure::new(positions, numbers, cell, pbc))
}

// #[wasm_bindgen]
// pub fn main(
//     positions: Vec<[f64; 3]>,
//     numbers: Vec<i32>,
//     cell: Vec<[Vec<[f64; 3]>; 3]>,
//     pbc: Vec<[bool; 3]>,
// ) {
//     let positions = vec![
//         [5., 6.395248, 5.],
//         [6.20832, 5.697624, 5.],
//         [6.20832, 4.302376, 5.],
//         [5., 3.604752, 5.],
//         [3.79168, 4.302376, 5.],
//         [3.79168, 5.697624, 5.],
//         [5., 7.48236, 5.],
//         [7.149787, 6.24118, 5.],
//         [7.149787, 3.75882, 5.],
//         [5., 2.51764, 5.],
//         [2.850213, 3.75882, 5.],
//         [2.850213, 6.24118, 5.],
//     ];
//     let numbers = vec![6, 6, 6, 6, 6, 6, 1, 1, 1, 1, 1, 1];
//     let cell = [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]];
//     let pbc = [false, false, false];

//     let structure = Structure::new(positions, numbers, cell, pbc);

//     let config = ViewerConfig {
//         color: ColorConfig {
//             background: Color::srgb(0.0, 0.0, 0.0),
//             cell_color: Color::srgb(0.9, 0.0, 0.0),
//         },
//         render: RenderConfig {
//             show_ui: true,
//             ..Default::default()
//         },
//         ..Default::default()
//     };

//     run_structure(structure, config);
// }
