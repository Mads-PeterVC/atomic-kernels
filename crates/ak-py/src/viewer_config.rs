use ak_vis::viewer::config::{ColorConfig, LightingConfig, RenderConfig, ViewerConfig};
use bevy::color::Color;
use pyo3::prelude::*;
use std::ops::Deref;

/// Python wrapper for LightingConfig
#[pyclass(name = "LightingConfig")]
#[derive(Clone)]
pub struct PyLightingConfig {
    #[pyo3(get, set)]
    pub ambient_brightness: f32,
    #[pyo3(get, set)]
    pub key_illuminance: f32,
    #[pyo3(get, set)]
    pub fill_illuminance: f32,
    #[pyo3(get, set)]
    pub back_illuminance: f32,
    #[pyo3(get, set)]
    pub camera_illuminance: f32,
    #[pyo3(get, set)]
    pub enable_fog: bool,
}

#[pymethods]
impl PyLightingConfig {
    #[new]
    #[pyo3(signature = (ambient_brightness=100.0, key_illuminance=5000.0, fill_illuminance=1000.0, back_illuminance=800.0, camera_illuminance=2000.0, enable_fog=false))]
    fn new(
        ambient_brightness: f32,
        key_illuminance: f32,
        fill_illuminance: f32,
        back_illuminance: f32,
        camera_illuminance: f32,
        enable_fog: bool,
    ) -> Self {
        PyLightingConfig {
            ambient_brightness,
            key_illuminance,
            fill_illuminance,
            back_illuminance,
            camera_illuminance,
            enable_fog,
        }
    }
}

impl From<&LightingConfig> for PyLightingConfig {
    fn from(config: &LightingConfig) -> Self {
        PyLightingConfig {
            ambient_brightness: config.ambient_brightness,
            key_illuminance: config.key_illuminance,
            fill_illuminance: config.fill_illuminance,
            back_illuminance: config.back_illuminance,
            camera_illuminance: config.camera_illuminance,
            enable_fog: config.enable_fog,
        }
    }
}

impl From<&PyLightingConfig> for LightingConfig {
    fn from(py: &PyLightingConfig) -> Self {
        LightingConfig {
            ambient_brightness: py.ambient_brightness,
            key_illuminance: py.key_illuminance,
            fill_illuminance: py.fill_illuminance,
            back_illuminance: py.back_illuminance,
            camera_illuminance: py.camera_illuminance,
            enable_fog: py.enable_fog,
        }
    }
}

impl Deref for PyLightingConfig {
    type Target = LightingConfig;

    fn deref(&self) -> &Self::Target {
        // Note: This creates a temporary on the stack
        // Safe because LightingConfig is Copy-like
        unsafe { &*(self as *const PyLightingConfig as *const LightingConfig) }
    }
}

/// Python wrapper for ColorConfig
/// Colors are represented as (r, g, b) tuples with values in [0.0, 1.0]
#[pyclass(name = "ColorConfig")]
#[derive(Clone)]
pub struct PyColorConfig {
    pub(crate) inner: ColorConfig,
}

#[pymethods]
impl PyColorConfig {
    #[new]
    #[pyo3(signature = (background=(0.98, 0.98, 0.98), cell_color=(0.0, 0.0, 0.0)))]
    fn new(background: (f32, f32, f32), cell_color: (f32, f32, f32)) -> PyResult<Self> {
        Ok(PyColorConfig {
            inner: ColorConfig {
                background: Color::srgb(background.0, background.1, background.2),
                cell_color: Color::srgb(cell_color.0, cell_color.1, cell_color.2),
            },
        })
    }

    #[getter]
    fn background(&self) -> (f32, f32, f32) {
        let c = self.inner.background.to_srgba();
        (c.red, c.green, c.blue)
    }

    #[setter]
    fn set_background(&mut self, value: (f32, f32, f32)) {
        self.inner.background = Color::srgb(value.0, value.1, value.2);
    }

    #[getter]
    fn cell_color(&self) -> (f32, f32, f32) {
        let c = self.inner.cell_color.to_srgba();
        (c.red, c.green, c.blue)
    }

    #[setter]
    fn set_cell_color(&mut self, value: (f32, f32, f32)) {
        self.inner.cell_color = Color::srgb(value.0, value.1, value.2);
    }
}

impl From<&ColorConfig> for PyColorConfig {
    fn from(config: &ColorConfig) -> Self {
        PyColorConfig {
            inner: config.clone(),
        }
    }
}

impl Deref for PyColorConfig {
    type Target = ColorConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Python wrapper for RenderConfig
#[pyclass(name = "RenderConfig")]
#[derive(Clone)]
pub struct PyRenderConfig {
    #[pyo3(get, set)]
    pub show_cell: bool,
    #[pyo3(get, set)]
    pub show_axes: bool,
    #[pyo3(get, set)]
    pub show_ui: bool,
    #[pyo3(get, set)]
    pub ico_subdiv: u32,
    #[pyo3(get, set)]
    pub show_orientation_widget: bool,
    #[pyo3(get, set)]
    pub orientation_widget_size_px: u32,
    #[pyo3(get, set)]
    pub orientation_widget_margin_px: u32,
    #[pyo3(get, set)]
    pub orientation_widget_offset_x_px: u32,
    #[pyo3(get, set)]
    pub orientation_widget_offset_y_px: u32,
    #[pyo3(get, set)]
    pub orientation_widget_camera_scale: f32,
}

#[pymethods]
impl PyRenderConfig {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (show_cell=true, show_axes=true, show_ui=false, ico_subdiv=5, show_orientation_widget=true, orientation_widget_size_px=100, orientation_widget_margin_px=0, orientation_widget_offset_x_px=None, orientation_widget_offset_y_px=None, orientation_widget_camera_scale=0.065))]
    fn new(
        show_cell: bool,
        show_axes: bool,
        show_ui: bool,
        ico_subdiv: u32,
        show_orientation_widget: bool,
        orientation_widget_size_px: u32,
        orientation_widget_margin_px: u32,
        orientation_widget_offset_x_px: Option<u32>,
        orientation_widget_offset_y_px: Option<u32>,
        orientation_widget_camera_scale: f32,
    ) -> Self {
        PyRenderConfig {
            show_cell,
            show_axes,
            show_ui,
            ico_subdiv,
            show_orientation_widget,
            orientation_widget_size_px,
            orientation_widget_margin_px,
            orientation_widget_offset_x_px: orientation_widget_offset_x_px
                .unwrap_or(orientation_widget_margin_px),
            orientation_widget_offset_y_px: orientation_widget_offset_y_px
                .unwrap_or(orientation_widget_margin_px),
            orientation_widget_camera_scale,
        }
    }
}

impl From<&RenderConfig> for PyRenderConfig {
    fn from(config: &RenderConfig) -> Self {
        PyRenderConfig {
            show_cell: config.show_cell,
            show_axes: config.show_axes,
            show_ui: config.show_ui,
            ico_subdiv: config.ico_subdiv,
            show_orientation_widget: config.show_orientation_widget,
            orientation_widget_size_px: config.orientation_widget_size_px,
            orientation_widget_margin_px: config.orientation_widget_margin_px,
            orientation_widget_offset_x_px: config.orientation_widget_offset_x_px,
            orientation_widget_offset_y_px: config.orientation_widget_offset_y_px,
            orientation_widget_camera_scale: config.orientation_widget_camera_scale,
        }
    }
}

impl From<&PyRenderConfig> for RenderConfig {
    fn from(py: &PyRenderConfig) -> Self {
        RenderConfig {
            show_cell: py.show_cell,
            show_axes: py.show_axes,
            show_ui: py.show_ui,
            ico_subdiv: py.ico_subdiv,
            show_orientation_widget: py.show_orientation_widget,
            orientation_widget_size_px: py.orientation_widget_size_px,
            orientation_widget_margin_px: py.orientation_widget_margin_px,
            orientation_widget_offset_x_px: py.orientation_widget_offset_x_px,
            orientation_widget_offset_y_px: py.orientation_widget_offset_y_px,
            orientation_widget_camera_scale: py.orientation_widget_camera_scale,
        }
    }
}

impl Deref for PyRenderConfig {
    type Target = RenderConfig;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const PyRenderConfig as *const RenderConfig) }
    }
}

/// Python wrapper for ViewerConfig
#[pyclass(name = "ViewerConfig")]
#[derive(Clone)]
pub struct PyViewerConfig {
    pub(crate) inner: ViewerConfig,
}

#[pymethods]
impl PyViewerConfig {
    #[new]
    #[pyo3(signature = (color=None, lighting=None, render=None, initial_frame=0, window_width=None, window_height=None))]
    fn new(
        color: Option<PyColorConfig>,
        lighting: Option<PyLightingConfig>,
        render: Option<PyRenderConfig>,
        initial_frame: usize,
        window_width: Option<u32>,
        window_height: Option<u32>,
    ) -> Self {
        let default_config = ViewerConfig::default();
        PyViewerConfig {
            inner: ViewerConfig {
                color: color.map(|c| c.inner).unwrap_or(default_config.color),
                lighting: lighting
                    .map(|l| (&l).into())
                    .unwrap_or(default_config.lighting),
                render: render.map(|r| (&r).into()).unwrap_or(default_config.render),
                initial_frame,
                window_width,
                window_height,
            },
        }
    }

    #[getter]
    fn color(&self) -> PyColorConfig {
        (&self.inner.color).into()
    }

    #[setter]
    fn set_color(&mut self, value: PyColorConfig) {
        self.inner.color = value.inner;
    }

    #[getter]
    fn lighting(&self) -> PyLightingConfig {
        (&self.inner.lighting).into()
    }

    #[setter]
    fn set_lighting(&mut self, value: PyLightingConfig) {
        self.inner.lighting = (&value).into();
    }

    #[getter]
    fn render(&self) -> PyRenderConfig {
        (&self.inner.render).into()
    }

    #[setter]
    fn set_render(&mut self, value: PyRenderConfig) {
        self.inner.render = (&value).into();
    }

    #[getter]
    fn initial_frame(&self) -> usize {
        self.inner.initial_frame
    }

    #[setter]
    fn set_initial_frame(&mut self, value: usize) {
        self.inner.initial_frame = value;
    }

    #[getter]
    fn window_width(&self) -> Option<u32> {
        self.inner.window_width
    }

    #[setter]
    fn set_window_width(&mut self, value: Option<u32>) {
        self.inner.window_width = value;
    }

    #[getter]
    fn window_height(&self) -> Option<u32> {
        self.inner.window_height
    }

    #[setter]
    fn set_window_height(&mut self, value: Option<u32>) {
        self.inner.window_height = value;
    }
}

impl Deref for PyViewerConfig {
    type Target = ViewerConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
