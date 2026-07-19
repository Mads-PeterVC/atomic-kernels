use super::helpers::*;
use crate::viewer::session::{CameraState, ViewerCommand, ViewerState, camera_view_for_frame};
use ak_core::Trajectory;
use bevy::prelude::Vec3;
#[test]
fn set_camera_view_updates_requested_fields() {
    let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    let mut camera = CameraState::new(&viewer);

    camera.apply_command(
        &viewer,
        &ViewerCommand::SetCameraView {
            focus: Some([1.0, 2.0, 3.0]),
            radius: Some(9.0),
            yaw: Some(0.5),
            pitch: Some(-0.25),
        },
    );

    assert_eq!(camera.focus, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(camera.radius, 9.0);
    assert_eq!(camera.yaw, 0.5);
    assert_eq!(camera.pitch, -0.25);
}

#[test]
fn orbit_and_zoom_camera_are_incremental() {
    let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    let mut camera = CameraState::new(&viewer);
    let initial = camera.clone();

    camera.apply_command(
        &viewer,
        &ViewerCommand::OrbitCamera {
            yaw_delta: 0.2,
            pitch_delta: -0.1,
        },
    );
    camera.apply_command(
        &viewer,
        &ViewerCommand::ZoomCamera {
            factor: Some(0.5),
            delta: None,
        },
    );

    assert_eq!(camera.yaw, initial.yaw + 0.2);
    assert_eq!(camera.pitch, initial.pitch - 0.1);
    assert_eq!(camera.radius, initial.radius * 0.5);
}

#[test]
fn start_and_stop_orbit_motion_updates_camera() {
    let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    let mut camera = CameraState::new(&viewer);
    let initial_yaw = camera.yaw;

    camera.apply_command(
        &viewer,
        &ViewerCommand::StartOrbit {
            yaw_rate: 1.0,
            pitch_rate: 0.0,
        },
    );
    camera.tick_motion(0.5);
    assert_eq!(camera.yaw, initial_yaw + 0.5);

    camera.apply_command(&viewer, &ViewerCommand::StopCameraMotion);
    camera.tick_motion(0.5);
    assert_eq!(camera.yaw, initial_yaw + 0.5);
}

#[test]
fn frame_all_restores_default_camera_view() {
    let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    let mut camera = CameraState::new(&viewer);
    camera.focus = Vec3::splat(5.0);
    camera.radius = 99.0;
    camera.yaw = 2.0;
    camera.pitch = 1.0;

    camera.apply_command(&viewer, &ViewerCommand::FrameAll);

    let expected = camera_view_for_frame(&viewer).unwrap();
    assert_eq!(camera.focus, expected.focus);
    assert_eq!(camera.radius, expected.radius);
    assert_eq!(camera.yaw, expected.yaw);
    assert_eq!(camera.pitch, expected.pitch);
}
