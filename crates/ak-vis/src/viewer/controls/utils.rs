use crate::viewer::{CameraState, ViewerState};

pub fn default_camera_state(viewer: &ViewerState) -> CameraState {
    CameraState::new(viewer)
}
