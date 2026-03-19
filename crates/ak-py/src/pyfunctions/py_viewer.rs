use crate::{PyStructure, PyTrajectory, PyViewerConfig};
#[cfg(not(target_os = "macos"))]
use ak_vis::launch;
use ak_vis::{
    BallAndStickStyle, BondList, BondScope, Face, FaceList, HeadlessRenderConfig, RenderStyle,
    ScalarColorMap, SelectedImageAtom, ViewerReadiness, ViewerSessionHandle, export_image,
    export_image_with_session, export_prepared_image, run, run_default, run_prepared,
    run_structure, run_structure_default, run_with_session,
};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use std::sync::Arc;
use std::time::Duration;

#[pyclass(name = "ViewerSession")]
pub struct PyViewerSession {
    handle: ViewerSessionHandle,
}

impl PyViewerSession {
    fn new(handle: ViewerSessionHandle) -> Self {
        Self { handle }
    }

    fn send_error(err: ak_vis::ViewerSessionClosed) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

#[pymethods]
impl PyViewerSession {
    fn append_frame(&self, frame: PyStructure) -> PyResult<()> {
        self.handle.append_frame(frame.0).map_err(Self::send_error)
    }

    fn set_frame(&self, index: usize) -> PyResult<()> {
        self.handle
            .set_current_frame(index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (enabled=true))]
    fn follow_tail(&self, enabled: bool) -> PyResult<()> {
        self.handle
            .set_follow_tail(enabled)
            .map_err(Self::send_error)
    }

    fn close(&self) -> PyResult<()> {
        self.handle.close().map_err(Self::send_error)
    }

    #[pyo3(signature = (timeout=None))]
    fn wait_until_ready(&self, timeout: Option<f64>) -> PyResult<bool> {
        let timeout = match timeout {
            Some(value) if !value.is_finite() || value < 0.0 => {
                return Err(PyValueError::new_err(
                    "timeout must be a finite non-negative number of seconds",
                ));
            }
            Some(value) => Some(Duration::from_secs_f64(value)),
            None => None,
        };

        Ok(self.handle.wait_until_ready(timeout))
    }

    #[pyo3(signature = (name, values, frame_index=None))]
    fn set_atom_scalars(
        &self,
        name: String,
        values: Vec<f32>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        self.handle
            .set_atom_scalars(name, values, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (name, palette="viridis", colors=None, min=None, max=None, append=false))]
    fn color_by_scalar(
        &self,
        name: String,
        palette: &str,
        colors: Option<Vec<(f32, f32, f32, f32)>>,
        min: Option<f32>,
        max: Option<f32>,
        append: bool,
    ) -> PyResult<()> {
        let palette = match colors {
            Some(colors) => {
                if colors.len() < 2 {
                    return Err(PyValueError::new_err(
                        "custom colormap requires at least two RGBA samples",
                    ));
                }
                ScalarColorMap::Sampled(
                    colors
                        .into_iter()
                        .map(|(r, g, b, a)| [r, g, b, a])
                        .collect(),
                )
            }
            None => match palette {
                "viridis" => ScalarColorMap::Viridis,
                "inferno" => ScalarColorMap::Inferno,
                "plasma" => ScalarColorMap::Plasma,
                _ => {
                    return Err(PyValueError::new_err(format!(
                        "unsupported palette '{palette}', expected 'viridis', 'inferno', or 'plasma'"
                    )));
                }
            },
        };

        self.handle
            .color_by_scalar(name, palette, min, max, append)
            .map_err(Self::send_error)
    }

    fn reset_atom_colors(&self) -> PyResult<()> {
        self.handle.reset_atom_colors().map_err(Self::send_error)
    }

    #[pyo3(signature = (bonds, frame_index=None))]
    fn set_bonds(&self, bonds: Vec<(usize, usize)>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .set_bonds(BondList::new(bonds), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (bonds, frame_index=None))]
    fn add_bonds(&self, bonds: Vec<(usize, usize)>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .add_bonds(BondList::new(bonds), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (bonds, frame_index=None))]
    fn remove_bonds(&self, bonds: Vec<(usize, usize)>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .remove_bonds(BondList::new(bonds), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (frame_index=None))]
    fn clear_bonds(&self, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .clear_bonds(frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None))]
    fn set_faces(
        &self,
        faces: Vec<Vec<usize>>,
        color: (f32, f32, f32, f32),
        face_colors: Option<Vec<(f32, f32, f32, f32)>>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        let default_color = [color.0, color.1, color.2, color.3];
        let face_colors = match face_colors {
            Some(colors) => {
                if colors.len() != faces.len() {
                    return Err(PyValueError::new_err(
                        "face_colors must have the same length as faces",
                    ));
                }
                colors
                    .into_iter()
                    .map(|(r, g, b, a)| [r, g, b, a])
                    .collect::<Vec<_>>()
            }
            None => vec![default_color; faces.len()],
        };

        let normalized = FaceList::new(
            faces
                .into_iter()
                .zip(face_colors)
                .filter_map(|(atoms, color)| Face::new(atoms, color)),
        );
        self.handle
            .set_faces(normalized, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None))]
    fn add_faces(
        &self,
        faces: Vec<Vec<usize>>,
        color: (f32, f32, f32, f32),
        face_colors: Option<Vec<(f32, f32, f32, f32)>>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        let normalized = normalize_faces(faces, color, face_colors)?;
        self.handle
            .add_faces(normalized, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (faces, color=(0.2, 0.6, 0.9, 0.35), face_colors=None, frame_index=None))]
    fn remove_faces(
        &self,
        faces: Vec<Vec<usize>>,
        color: (f32, f32, f32, f32),
        face_colors: Option<Vec<(f32, f32, f32, f32)>>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        let normalized = normalize_faces(faces, color, face_colors)?;
        self.handle
            .remove_faces(normalized, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (frame_index=None))]
    fn clear_faces(&self, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .clear_faces(frame_index)
            .map_err(Self::send_error)
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (selection, atom_scale=0.45, bond_radius=0.08, bond_color=(0.7, 0.7, 0.7, 1.0), bond_scope="both_selected", frame_index=None, append=false))]
    fn set_ball_and_stick_style(
        &self,
        selection: Vec<bool>,
        atom_scale: f32,
        bond_radius: f32,
        bond_color: (f32, f32, f32, f32),
        bond_scope: &str,
        frame_index: Option<usize>,
        append: bool,
    ) -> PyResult<()> {
        let bond_scope = match bond_scope {
            "both_selected" => BondScope::BothSelected,
            "touch_selection" => BondScope::TouchSelection,
            _ => {
                return Err(PyValueError::new_err(
                    "unsupported bond_scope, expected 'both_selected' or 'touch_selection'",
                ));
            }
        };
        self.handle
            .set_render_style(
                RenderStyle::BallAndStick(BallAndStickStyle {
                    atom_scale,
                    bond_radius,
                    bond_color: [bond_color.0, bond_color.1, bond_color.2, bond_color.3],
                    bond_scope,
                }),
                selection,
                frame_index,
                append,
            )
            .map_err(Self::send_error)
    }

    fn reset_render_style(&self) -> PyResult<()> {
        self.handle.reset_render_style().map_err(Self::send_error)
    }

    #[pyo3(signature = (frame_index=None))]
    fn selected_atoms(&self, frame_index: Option<usize>) -> Vec<usize> {
        self.handle.selected_atoms(frame_index)
    }

    #[pyo3(signature = (frame_index=None))]
    fn selected_images(&self, frame_index: Option<usize>) -> Vec<(usize, (i32, i32, i32))> {
        self.handle
            .selected_images(frame_index)
            .into_iter()
            .map(|atom| {
                (
                    atom.atom_index,
                    (
                        atom.image_offset[0],
                        atom.image_offset[1],
                        atom.image_offset[2],
                    ),
                )
            })
            .collect()
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn set_selection(&self, selection: Vec<bool>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .replace_selection(selection, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn add_selection(&self, selection: Vec<bool>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .add_selection(selection, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn remove_selection(&self, selection: Vec<bool>, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .remove_selection(selection, frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (frame_index=None))]
    fn clear_selection(&self, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .clear_selection(frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn set_image_selection(
        &self,
        selection: Vec<(usize, (i32, i32, i32))>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        self.handle
            .replace_image_selection(normalize_image_selection(selection), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn add_image_selection(
        &self,
        selection: Vec<(usize, (i32, i32, i32))>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        self.handle
            .add_image_selection(normalize_image_selection(selection), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (selection, frame_index=None))]
    fn remove_image_selection(
        &self,
        selection: Vec<(usize, (i32, i32, i32))>,
        frame_index: Option<usize>,
    ) -> PyResult<()> {
        self.handle
            .remove_image_selection(normalize_image_selection(selection), frame_index)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (frame_index=None))]
    fn clear_image_selection(&self, frame_index: Option<usize>) -> PyResult<()> {
        self.handle
            .clear_image_selection(frame_index)
            .map_err(Self::send_error)
    }

    fn set_supercell(&self, repeats: (u32, u32, u32)) -> PyResult<()> {
        self.handle
            .set_supercell([repeats.0, repeats.1, repeats.2])
            .map_err(Self::send_error)
    }

    fn increment_supercell_axis(&self, axis: usize) -> PyResult<()> {
        self.handle
            .increment_supercell_axis(axis)
            .map_err(Self::send_error)
    }

    fn decrement_supercell_axis(&self, axis: usize) -> PyResult<()> {
        self.handle
            .decrement_supercell_axis(axis)
            .map_err(Self::send_error)
    }

    fn reset_supercell(&self) -> PyResult<()> {
        self.handle.reset_supercell().map_err(Self::send_error)
    }

    #[pyo3(signature = (enabled=true))]
    fn set_ghost_repeated_images(&self, enabled: bool) -> PyResult<()> {
        self.handle
            .set_ghost_repeated_images(enabled)
            .map_err(Self::send_error)
    }

    fn toggle_ghost_repeated_images(&self) -> PyResult<()> {
        self.handle
            .toggle_ghost_repeated_images()
            .map_err(Self::send_error)
    }

    fn supercell(&self) -> ((u32, u32, u32), bool) {
        let settings = self.handle.supercell();
        (
            (
                settings.repeats[0],
                settings.repeats[1],
                settings.repeats[2],
            ),
            settings.ghost_repeated_images,
        )
    }

    #[pyo3(signature = (focus=None, radius=None, yaw=None, pitch=None))]
    fn set_camera_view(
        &self,
        focus: Option<[f32; 3]>,
        radius: Option<f32>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    ) -> PyResult<()> {
        self.handle
            .set_camera_view(focus, radius, yaw, pitch)
            .map_err(Self::send_error)
    }

    fn pan_camera(&self, delta: [f32; 3]) -> PyResult<()> {
        self.handle.pan_camera(delta).map_err(Self::send_error)
    }

    #[pyo3(signature = (factor=None, delta=None))]
    fn zoom_camera(&self, factor: Option<f32>, delta: Option<f32>) -> PyResult<()> {
        self.handle
            .zoom_camera(factor, delta)
            .map_err(Self::send_error)
    }

    #[pyo3(signature = (yaw_delta=0.0, pitch_delta=0.0))]
    fn orbit_camera(&self, yaw_delta: f32, pitch_delta: f32) -> PyResult<()> {
        self.handle
            .orbit_camera(yaw_delta, pitch_delta)
            .map_err(Self::send_error)
    }

    fn frame_all(&self) -> PyResult<()> {
        self.handle.frame_all().map_err(Self::send_error)
    }

    #[pyo3(signature = (yaw_rate=0.5, pitch_rate=0.0))]
    fn start_orbit(&self, yaw_rate: f32, pitch_rate: f32) -> PyResult<()> {
        self.handle
            .start_orbit(yaw_rate, pitch_rate)
            .map_err(Self::send_error)
    }

    fn stop_camera_motion(&self) -> PyResult<()> {
        self.handle.stop_camera_motion().map_err(Self::send_error)
    }
}

fn normalize_faces(
    faces: Vec<Vec<usize>>,
    color: (f32, f32, f32, f32),
    face_colors: Option<Vec<(f32, f32, f32, f32)>>,
) -> PyResult<FaceList> {
    let default_color = [color.0, color.1, color.2, color.3];
    let face_colors = match face_colors {
        Some(colors) => {
            if colors.len() != faces.len() {
                return Err(PyValueError::new_err(
                    "face_colors must have the same length as faces",
                ));
            }
            colors
                .into_iter()
                .map(|(r, g, b, a)| [r, g, b, a])
                .collect::<Vec<_>>()
        }
        None => vec![default_color; faces.len()],
    };

    Ok(FaceList::new(
        faces
            .into_iter()
            .zip(face_colors)
            .filter_map(|(atoms, color)| Face::new(atoms, color)),
    ))
}

fn normalize_image_selection(selection: Vec<(usize, (i32, i32, i32))>) -> Vec<SelectedImageAtom> {
    selection
        .into_iter()
        .map(|(atom_index, image_offset)| SelectedImageAtom {
            atom_index,
            image_offset: [image_offset.0, image_offset.1, image_offset.2],
        })
        .collect()
}

#[pyclass(name = "PreparedViewerSession", unsendable)]
pub struct PyPreparedViewerSession {
    handle: ViewerSessionHandle,
    trajectory: Option<ak_core::Trajectory>,
    config: Option<ak_vis::viewer::ViewerConfig>,
    receiver: Option<std::sync::mpsc::Receiver<ak_vis::ViewerCommand>>,
}

#[pyclass(name = "PreparedHeadlessRender", unsendable)]
pub struct PyPreparedHeadlessRender {
    handle: ViewerSessionHandle,
    trajectory: Option<ak_core::Trajectory>,
    config: Option<ak_vis::viewer::ViewerConfig>,
    export: Option<HeadlessRenderConfig>,
    receiver: Option<std::sync::mpsc::Receiver<ak_vis::ViewerCommand>>,
}

#[pymethods]
impl PyPreparedViewerSession {
    #[getter]
    fn session(&self) -> PyViewerSession {
        PyViewerSession::new(self.handle.clone())
    }

    fn run(&mut self, py: Python<'_>) -> PyResult<()> {
        let handle = self.handle.clone();
        let trajectory = self.trajectory.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared viewer session has already been run")
        })?;
        let config = self.config.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared viewer session has already been run")
        })?;
        let receiver = self.receiver.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared viewer session has already been run")
        })?;

        py.detach(move || {
            run_prepared(
                trajectory,
                config,
                receiver,
                handle.readiness().clone(),
                handle.snapshot().clone(),
            );
        });

        Ok(())
    }
}

#[pymethods]
impl PyPreparedHeadlessRender {
    #[getter]
    fn session(&self) -> PyViewerSession {
        PyViewerSession::new(self.handle.clone())
    }

    fn save(&mut self, py: Python<'_>) -> PyResult<()> {
        let trajectory = self.trajectory.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared headless render has already been saved")
        })?;
        let config = self.config.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared headless render has already been saved")
        })?;
        let export = self.export.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared headless render has already been saved")
        })?;
        let receiver = self.receiver.take().ok_or_else(|| {
            PyRuntimeError::new_err("prepared headless render has already been saved")
        })?;

        py.detach(move || export_prepared_image(trajectory, config, export, receiver))
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))
    }
}

fn ensure_non_empty_trajectory(trajectory: &PyTrajectory) -> PyResult<()> {
    if trajectory.0.is_empty() {
        return Err(PyValueError::new_err(
            "viewer session requires at least one frame",
        ));
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (structure, config=None))]
pub fn viewer(_py: Python<'_>, structure: PyStructure, config: Option<PyViewerConfig>) {
    match config {
        Some(cfg) => run_structure(structure.0, cfg.inner.clone()),
        None => run_structure_default(structure.0),
    }
}

#[pyfunction]
#[pyo3(signature = (trajectory, config=None))]
pub fn trajectory_viewer(
    _py: Python<'_>,
    trajectory: PyTrajectory,
    config: Option<PyViewerConfig>,
) -> PyResult<()> {
    ensure_non_empty_trajectory(&trajectory)?;

    match config {
        Some(cfg) => run(trajectory.0, cfg.inner.clone()),
        None => run_default(trajectory.0),
    }

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (trajectory, config=None))]
#[cfg(not(target_os = "macos"))]
pub fn launch_viewer(
    _py: Python<'_>,
    trajectory: PyTrajectory,
    config: Option<PyViewerConfig>,
) -> PyResult<PyViewerSession> {
    ensure_non_empty_trajectory(&trajectory)?;

    let handle = match config {
        Some(cfg) => launch(trajectory.0, cfg.inner.clone()),
        None => launch(trajectory.0, Default::default()),
    };

    Ok(PyViewerSession { handle })
}

#[pyfunction]
#[pyo3(signature = (trajectory, config=None))]
#[cfg(target_os = "macos")]
pub fn launch_viewer(
    _py: Python<'_>,
    trajectory: PyTrajectory,
    config: Option<PyViewerConfig>,
) -> PyResult<PyViewerSession> {
    let _ = trajectory;
    let _ = config;
    Err(PyRuntimeError::new_err(
        "launch_viewer is not supported on macOS because Bevy's event loop must run on the main thread; use run_viewer_session(trajectory, callback, config) instead",
    ))
}

#[pyfunction]
#[pyo3(signature = (trajectory, callback, config=None))]
pub fn run_viewer_session(
    py: Python<'_>,
    trajectory: PyTrajectory,
    callback: Py<PyAny>,
    config: Option<PyViewerConfig>,
) -> PyResult<()> {
    ensure_non_empty_trajectory(&trajectory)?;
    let config = config.map(|cfg| cfg.inner.clone()).unwrap_or_default();

    py.detach(move || {
        run_with_session(trajectory.0, config, move |handle| {
            Python::attach(|py| {
                let session = match Py::new(py, PyViewerSession::new(handle)) {
                    Ok(session) => session,
                    Err(err) => {
                        err.print(py);
                        return;
                    }
                };

                if let Err(err) = callback.call1(py, (session,)) {
                    err.print(py);
                }
            });
        });
    });

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (trajectory, config=None))]
pub fn prepare_viewer_session(
    _py: Python<'_>,
    trajectory: PyTrajectory,
    config: Option<PyViewerConfig>,
) -> PyResult<PyPreparedViewerSession> {
    ensure_non_empty_trajectory(&trajectory)?;

    let (sender, receiver) = std::sync::mpsc::channel();
    let readiness = Arc::new(ViewerReadiness::new());
    Ok(PyPreparedViewerSession {
        handle: ViewerSessionHandle::with_readiness(sender, readiness),
        trajectory: Some(trajectory.0),
        config: Some(config.map(|cfg| cfg.inner.clone()).unwrap_or_default()),
        receiver: Some(receiver),
    })
}

#[pyfunction]
#[pyo3(signature = (trajectory, path, width=800, height=600, callback=None, config=None))]
pub fn render_viewer_image(
    py: Python<'_>,
    trajectory: PyTrajectory,
    path: String,
    width: u32,
    height: u32,
    callback: Option<Py<PyAny>>,
    config: Option<PyViewerConfig>,
) -> PyResult<()> {
    ensure_non_empty_trajectory(&trajectory)?;

    let config = config.map(|cfg| cfg.inner.clone()).unwrap_or_default();
    let export = HeadlessRenderConfig::new(path, width, height);

    match callback {
        Some(callback) => py.detach(move || {
            export_image_with_session(trajectory.0, config, export, move |handle| {
                Python::attach(|py| {
                    let session = match Py::new(py, PyViewerSession::new(handle)) {
                        Ok(session) => session,
                        Err(err) => {
                            err.print(py);
                            return;
                        }
                    };

                    if let Err(err) = callback.call1(py, (session,)) {
                        err.print(py);
                    }
                });
            })
        }),
        None => py.detach(move || export_image(trajectory.0, config, export)),
    }
    .map_err(|err| PyRuntimeError::new_err(err.to_string()))
}

#[pyfunction]
#[pyo3(signature = (trajectory, path, width=800, height=600, config=None))]
pub fn prepare_render_viewer_image(
    _py: Python<'_>,
    trajectory: PyTrajectory,
    path: String,
    width: u32,
    height: u32,
    config: Option<PyViewerConfig>,
) -> PyResult<PyPreparedHeadlessRender> {
    ensure_non_empty_trajectory(&trajectory)?;

    let (sender, receiver) = std::sync::mpsc::channel();
    let readiness = Arc::new(ViewerReadiness::new());
    Ok(PyPreparedHeadlessRender {
        handle: ViewerSessionHandle::with_readiness(sender, readiness),
        trajectory: Some(trajectory.0),
        config: Some(config.map(|cfg| cfg.inner.clone()).unwrap_or_default()),
        export: Some(HeadlessRenderConfig::new(path, width, height)),
        receiver: Some(receiver),
    })
}
