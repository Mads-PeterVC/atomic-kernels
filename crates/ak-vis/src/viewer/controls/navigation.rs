use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::Key;
use bevy::prelude::*;

use crate::components::{
    FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameMeasurementCue,
    FrameSelectionHighlight,
};
use crate::viewer::ViewerState;

#[derive(SystemParam)]
pub struct RenderResources<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    logical_keys: Res<'w, ButtonInput<Key>>,
    time: Res<'w, Time>,
    viewer: ResMut<'w, ViewerState>,
}

fn digit_hotkey_pressed(
    physical_keys: &ButtonInput<KeyCode>,
    logical_keys: &ButtonInput<Key>,
    physical: KeyCode,
    text: &'static str,
) -> bool {
    physical_keys.just_pressed(physical) || logical_keys.just_pressed(Key::Character(text.into()))
}

pub fn despawn_current_frame(
    commands: &mut Commands,
    atoms: Query<Entity, With<FrameAtom>>,
    selection_highlights: Query<Entity, With<FrameSelectionHighlight>>,
    measurement_cues: Query<Entity, With<FrameMeasurementCue>>,
    cells: Query<Entity, With<FrameCell>>,
    axes: Query<Entity, With<FrameAxis>>,
    bonds: Query<Entity, With<FrameBond>>,
    faces: Query<Entity, With<FrameFace>>,
) {
    for entity in atoms.iter() {
        commands.entity(entity).despawn();
    }
    for entity in selection_highlights.iter() {
        commands.entity(entity).despawn();
    }
    for entity in measurement_cues.iter() {
        commands.entity(entity).despawn();
    }
    for entity in cells.iter() {
        commands.entity(entity).despawn();
    }
    for entity in axes.iter() {
        commands.entity(entity).despawn();
    }
    for entity in bonds.iter() {
        commands.entity(entity).despawn();
    }
    for entity in faces.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn navigate_frames(mut timer: Local<Timer>, mut resources: RenderResources) {
    let shifted = resources
        .keys
        .any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    for (axis, key, text) in [
        (0usize, KeyCode::Digit1, "1"),
        (1usize, KeyCode::Digit2, "2"),
        (2usize, KeyCode::Digit3, "3"),
    ] {
        if digit_hotkey_pressed(&resources.keys, &resources.logical_keys, key, text) {
            let _ = resources.viewer.apply_command(if shifted {
                crate::viewer::ViewerCommand::DecrementSupercellAxis { axis }
            } else {
                crate::viewer::ViewerCommand::IncrementSupercellAxis { axis }
            });
        }
    }

    if digit_hotkey_pressed(
        &resources.keys,
        &resources.logical_keys,
        KeyCode::Digit0,
        "0",
    ) {
        let _ = resources
            .viewer
            .apply_command(crate::viewer::ViewerCommand::ToggleGhostRepeatedImages);
    }

    // Initialize timer on first run (0.1 seconds = 10 frames per second)
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    }

    timer.tick(resources.time.delta());

    // Only advance frame when timer finishes
    if !timer.just_finished() {
        return;
    }

    if resources.keys.pressed(KeyCode::KeyD) {
        let _ = resources.viewer.step_frame(1);
    } else if resources.keys.pressed(KeyCode::KeyA) {
        let _ = resources.viewer.step_frame(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::{despawn_current_frame, navigate_frames};
    use crate::components::{
        FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameMeasurementCue,
        FrameSelectionHighlight,
    };
    use crate::viewer::ViewerState;
    use ak_core::{Structure, Trajectory};
    use bevy::ecs::system::SystemState;
    use bevy::input::keyboard::Key;
    use bevy::prelude::*;
    use std::time::Duration;

    fn sample_viewer() -> ViewerState {
        let frames = vec![
            Structure::new(
                vec![[0.0, 0.0, 0.0]],
                vec![1],
                [[5.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 5.0]],
                [false; 3],
            ),
            Structure::new(
                vec![[1.0, 0.0, 0.0]],
                vec![1],
                [[5.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 5.0]],
                [false; 3],
            ),
        ];
        ViewerState::new(Trajectory::new(frames), 0)
    }

    #[test]
    fn despawn_current_frame_removes_frame_entities() {
        let mut world = World::new();
        let atom = world.spawn(FrameAtom).id();
        let highlight = world.spawn(FrameSelectionHighlight).id();
        let measurement = world.spawn(FrameMeasurementCue).id();
        let cell = world.spawn(FrameCell).id();
        let axis = world.spawn(FrameAxis).id();
        let bond = world.spawn(FrameBond).id();
        let face = world.spawn(FrameFace).id();

        let mut system_state: SystemState<(
            Commands,
            Query<Entity, With<FrameAtom>>,
            Query<Entity, With<FrameSelectionHighlight>>,
            Query<Entity, With<FrameMeasurementCue>>,
            Query<Entity, With<FrameCell>>,
            Query<Entity, With<FrameAxis>>,
            Query<Entity, With<FrameBond>>,
            Query<Entity, With<FrameFace>>,
        )> = SystemState::new(&mut world);

        let (
            mut commands,
            atoms,
            selection_highlights,
            measurement_cues,
            cells,
            axes,
            bonds,
            faces,
        ) =
            system_state.get_mut(&mut world);
        despawn_current_frame(
            &mut commands,
            atoms,
            selection_highlights,
            measurement_cues,
            cells,
            axes,
            bonds,
            faces,
        );
        system_state.apply(&mut world);

        assert!(world.get_entity(atom).is_err());
        assert!(world.get_entity(highlight).is_err());
        assert!(world.get_entity(measurement).is_err());
        assert!(world.get_entity(cell).is_err());
        assert!(world.get_entity(axis).is_err());
        assert!(world.get_entity(bond).is_err());
        assert!(world.get_entity(face).is_err());
    }

    #[test]
    fn navigate_frames_steps_when_timer_finishes() {
        let mut app = App::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyD);
        let logical_keys = ButtonInput::<Key>::default();

        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(0.11));

        app.insert_resource(keys);
        app.insert_resource(logical_keys);
        app.insert_resource(time);
        app.insert_resource(sample_viewer());
        app.add_systems(Update, navigate_frames);

        app.update();

        assert_eq!(app.world().resource::<ViewerState>().current, 1);
    }

    #[test]
    fn navigate_frames_adjusts_supercell_and_ghosting_with_digit_hotkeys() {
        let mut app = App::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Digit1);
        keys.press(KeyCode::Digit0);
        let logical_keys = ButtonInput::<Key>::default();

        let time = Time::<()>::default();

        app.insert_resource(keys);
        app.insert_resource(logical_keys);
        app.insert_resource(time);
        app.insert_resource(sample_viewer());
        app.add_systems(Update, navigate_frames);

        app.update();

        let viewer = app.world().resource::<ViewerState>();
        assert_eq!(viewer.supercell.repeats, [1, 0, 0]);
        assert!(!viewer.supercell.ghost_repeated_images);
    }

    #[test]
    fn navigate_frames_accepts_logical_digit_hotkeys() {
        let mut app = App::new();
        let keys = ButtonInput::<KeyCode>::default();
        let mut logical_keys = ButtonInput::<Key>::default();
        logical_keys.press(Key::Character("1".into()));
        logical_keys.press(Key::Character("0".into()));

        let time = Time::<()>::default();

        app.insert_resource(keys);
        app.insert_resource(logical_keys);
        app.insert_resource(time);
        app.insert_resource(sample_viewer());
        app.add_systems(Update, navigate_frames);

        app.update();

        let viewer = app.world().resource::<ViewerState>();
        assert_eq!(viewer.supercell.repeats, [1, 0, 0]);
        assert!(!viewer.supercell.ghost_repeated_images);
    }
}
