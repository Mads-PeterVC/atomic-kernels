use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct ViewerConfig {
    // Colors
    pub background: Color,
    pub cell_color: Color,
    // Lightning
    pub ambient_brightness: f32,
    pub key_illuminance: f32,
    pub fill_illuminance: f32,
    pub back_illuminance: f32,
    pub camera_illuminance: f32,

    // What to render
    pub show_cell: bool,
    pub show_axes: bool,
    // Other
    pub initial_frame: usize,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.98, 0.98, 0.98),
            cell_color: Color::srgb(0.0, 0.0, 0.0),

            ambient_brightness: 100.0,
            key_illuminance: 5_000.0,
            fill_illuminance: 1_000.0,
            back_illuminance: 800.0,
            camera_illuminance: 2_000.0,

            show_cell: true,
            show_axes: true,
            initial_frame: 0,
        }
    }
}
