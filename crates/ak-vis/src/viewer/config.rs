use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct ViewerConfig {
    pub background: Color,
    pub ambient_brightness: f32,
    pub directional_illuminance: f32,
    pub show_cell: bool,
    pub show_axes: bool,
    pub initial_frame: usize,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.98, 0.98, 0.98),
            ambient_brightness: 1000.0,
            directional_illuminance: 1_000.0,
            show_cell: true,
            show_axes: true,
            initial_frame: 0,
        }
    }
}
