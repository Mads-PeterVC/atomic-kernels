use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::viewer::{CameraState, ViewerState};

pub fn toggle_view(
    keys: Res<ButtonInput<KeyCode>>,
    viewer: Res<ViewerState>,
    mut camera: ResMut<CameraState>,
) {
    let (target_yaw, target_pitch) = if keys.just_pressed(KeyCode::KeyX) {
        (-FRAC_PI_2, 0.0)
    } else if keys.just_pressed(KeyCode::KeyY) {
        (0.0, -FRAC_PI_2)
    } else if keys.just_pressed(KeyCode::KeyZ) {
        (0.0, 0.0)
    } else {
        return;
    };

    camera.yaw = target_yaw;
    camera.pitch = target_pitch;
    camera.reset_for_frame(viewer.as_ref());
    camera.yaw = target_yaw;
    camera.pitch = target_pitch;
    camera.motion = None;
    camera.needs_apply = true;
}

pub fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<CameraState>,
) {
    let mut changed = false;

    if key_input.pressed(KeyCode::ArrowRight) {
        camera.yaw += 50f32.to_radians() * time.delta_secs();
        changed = true;
    }
    if key_input.pressed(KeyCode::ArrowLeft) {
        camera.yaw -= 50f32.to_radians() * time.delta_secs();
        changed = true;
    }
    if key_input.pressed(KeyCode::ArrowUp) {
        camera.pitch += 50f32.to_radians() * time.delta_secs();
        changed = true;
    }
    if key_input.pressed(KeyCode::ArrowDown) {
        camera.pitch -= 50f32.to_radians() * time.delta_secs();
        changed = true;
    }
    if key_input.pressed(KeyCode::KeyW) {
        camera.radius = (camera.radius - 5.0 * time.delta_secs()).max(f32::EPSILON);
        changed = true;
    }
    if key_input.pressed(KeyCode::KeyS) {
        camera.radius += 5.0 * time.delta_secs();
        changed = true;
    }

    if changed {
        camera.motion = None;
        camera.needs_apply = true;
    }
}
