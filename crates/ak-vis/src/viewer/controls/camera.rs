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

#[cfg(test)]
mod tests {
    use super::{keyboard_controls, toggle_view};
    use crate::viewer::{CameraState, ViewerState};
    use ak_core::{Structure, Trajectory};
    use bevy::ecs::system::SystemState;
    use bevy::prelude::*;
    use std::time::Duration;

    fn sample_viewer() -> ViewerState {
        let structure = Structure::new(
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![1, 1],
            [[5.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 5.0]],
            [false; 3],
        );
        ViewerState::new(Trajectory::new(vec![structure]), 0)
    }

    #[test]
    fn toggle_view_snaps_camera_to_requested_axis() {
        let mut world = World::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyX);

        let viewer = sample_viewer();
        let initial_focus = CameraState::new(&viewer).focus;

        world.insert_resource(keys);
        world.insert_resource(viewer);
        world.insert_resource(CameraState::new(world.resource::<ViewerState>()));

        let mut system_state: SystemState<(
            Res<ButtonInput<KeyCode>>,
            Res<ViewerState>,
            ResMut<CameraState>,
        )> = SystemState::new(&mut world);

        let (keys, viewer, camera) = system_state.get_mut(&mut world);
        toggle_view(keys, viewer, camera);
        system_state.apply(&mut world);

        let camera = world.resource::<CameraState>();
        assert_eq!(camera.yaw, -std::f32::consts::FRAC_PI_2);
        assert_eq!(camera.pitch, 0.0);
        assert!(camera.needs_apply);
        assert_eq!(camera.motion, None);
        assert_eq!(camera.focus, initial_focus);
    }

    #[test]
    fn keyboard_controls_adjust_camera_motion() {
        let mut world = World::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::ArrowRight);
        keys.press(KeyCode::ArrowUp);
        keys.press(KeyCode::KeyW);

        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(0.5));

        let viewer = sample_viewer();
        let mut camera = CameraState::new(&viewer);
        let initial_yaw = camera.yaw;
        let initial_pitch = camera.pitch;
        let initial_radius = camera.radius;
        camera.motion = Some(crate::viewer::session::OrbitMotion {
            yaw_rate: 1.0,
            pitch_rate: 1.0,
        });
        camera.needs_apply = false;

        world.insert_resource(time);
        world.insert_resource(keys);
        world.insert_resource(camera);

        let mut system_state: SystemState<(
            Res<Time>,
            Res<ButtonInput<KeyCode>>,
            ResMut<CameraState>,
        )> = SystemState::new(&mut world);

        let (time, keys, camera) = system_state.get_mut(&mut world);
        keyboard_controls(time, keys, camera);
        system_state.apply(&mut world);

        let camera = world.resource::<CameraState>();
        assert!(camera.yaw > initial_yaw);
        assert!(camera.pitch > initial_pitch);
        assert!(camera.radius < initial_radius);
        assert!(camera.motion.is_none());
        assert!(camera.needs_apply);
    }
}
