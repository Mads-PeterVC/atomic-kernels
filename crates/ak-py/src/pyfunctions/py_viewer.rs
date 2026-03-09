use crate::{PyStructure, PyTrajectory, PyViewerConfig};
#[cfg(not(target_os = "macos"))]
use ak_vis::launch;
use ak_vis::{
    ViewerSessionHandle, run, run_default, run_prepared, run_structure, run_structure_default,
    run_with_session,
};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

#[pyclass(name = "ViewerSession")]
pub struct PyViewerSession {
    handle: ViewerSessionHandle,
}

impl PyViewerSession {
    fn new(handle: ViewerSessionHandle) -> Self {
        Self { handle }
    }

    fn send_error(err: std::sync::mpsc::SendError<ak_vis::ViewerCommand>) -> PyErr {
        PyRuntimeError::new_err(format!("viewer session is no longer available: {err}"))
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
}

#[pyclass(name = "PreparedViewerSession", unsendable)]
pub struct PyPreparedViewerSession {
    handle: ViewerSessionHandle,
    trajectory: Option<ak_core::Trajectory>,
    config: Option<ak_vis::viewer::ViewerConfig>,
    receiver: Option<std::sync::mpsc::Receiver<ak_vis::ViewerCommand>>,
}

#[pymethods]
impl PyPreparedViewerSession {
    #[getter]
    fn session(&self) -> PyViewerSession {
        PyViewerSession::new(self.handle.clone())
    }

    fn run(&mut self, py: Python<'_>) -> PyResult<()> {
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
            run_prepared(trajectory, config, receiver);
        });

        Ok(())
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
    Ok(PyPreparedViewerSession {
        handle: ViewerSessionHandle::new(sender),
        trajectory: Some(trajectory.0),
        config: Some(config.map(|cfg| cfg.inner.clone()).unwrap_or_default()),
        receiver: Some(receiver),
    })
}
