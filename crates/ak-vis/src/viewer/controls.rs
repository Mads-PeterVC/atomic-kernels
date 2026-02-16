use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::FRAC_PI_2;

pub fn toggle_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut cam_q: Query<(&mut PanOrbitCamera, &mut GlobalTransform)>,
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

    let Ok((mut orbit, cam_gt)) = cam_q.single_mut() else {
        return;
    };

    let radius = orbit.radius.unwrap_or_else(|| {
        (cam_gt.translation() - orbit.focus)
            .length()
            .max(orbit.zoom_lower_limit)
    });

    orbit.target_focus = orbit.focus;
    orbit.target_yaw = target_yaw;
    orbit.target_pitch = target_pitch;
    orbit.target_radius = radius;
    orbit.force_update = true;
}

pub fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        // Smooth rotation using arrow keys
        if key_input.pressed(KeyCode::ArrowRight) {
            pan_orbit.target_yaw += 50f32.to_radians() * time.delta_secs();
        }
        if key_input.pressed(KeyCode::ArrowLeft) {
            pan_orbit.target_yaw -= 50f32.to_radians() * time.delta_secs();
        }
        if key_input.pressed(KeyCode::ArrowUp) {
            pan_orbit.target_pitch += 50f32.to_radians() * time.delta_secs();
        }
        if key_input.pressed(KeyCode::ArrowDown) {
            pan_orbit.target_pitch -= 50f32.to_radians() * time.delta_secs();
        }

        // Zoom in with W and S
        if key_input.pressed(KeyCode::KeyW) {
            pan_orbit.target_radius -= 5.0 * time.delta_secs();
        }
        if key_input.pressed(KeyCode::KeyS) {
            pan_orbit.target_radius += 5.0 * time.delta_secs();
        }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}
