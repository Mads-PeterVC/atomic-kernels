use bevy::prelude::*;

#[derive(Clone)]
pub struct LightingConfig {
    pub ambient_brightness: f32,
    pub key_illuminance: f32,
    pub fill_illuminance: f32,
    pub back_illuminance: f32,
    pub camera_illuminance: f32,
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            ambient_brightness: 100.0,
            key_illuminance: 5_000.0,
            fill_illuminance: 1_000.0,
            back_illuminance: 800.0,
            camera_illuminance: 2_000.0,
        }
    }
}
#[derive(Clone)]
pub struct ColorConfig {
    pub background: Color,
    pub cell_color: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.98, 0.98, 0.98),
            cell_color: Color::srgb(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct RenderConfig {
    pub show_cell: bool,
    pub show_axes: bool,
    pub show_ui: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            show_cell: true,
            show_axes: true,
            show_ui: false,
        }
    }
}

#[derive(Clone, Resource)]
pub struct ViewerConfig {
    pub color: ColorConfig,
    pub lighting: LightingConfig,
    pub render: RenderConfig,
    pub initial_frame: usize,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            color: Default::default(),
            lighting: Default::default(),
            render: Default::default(),
            initial_frame: 0,
        }
    }
}
