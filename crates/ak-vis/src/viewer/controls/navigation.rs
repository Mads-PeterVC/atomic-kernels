use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::Key;
use bevy::prelude::*;

use crate::components::{
    FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameMeasurementCue,
    FrameSelectionHighlight,
};
use crate::ui::PlaybackState;
use crate::viewer::ViewerState;

#[derive(SystemParam)]
pub struct RenderResources<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    logical_keys: Res<'w, ButtonInput<Key>>,
    time: Res<'w, Time>,
    viewer: ResMut<'w, ViewerState>,
    playback: ResMut<'w, PlaybackState>,
}

#[derive(Default)]
pub struct FrameStepRepeatState {
    forward: KeyRepeat,
    backward: KeyRepeat,
}

#[derive(Default)]
struct KeyRepeat {
    timer: Option<Timer>,
    repeating: bool,
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

pub fn navigate_frames(
    mut timer: Local<Timer>,
    mut key_repeat: Local<FrameStepRepeatState>,
    mut resources: RenderResources,
) {
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

    // Initialize timer on first run; the duration is replaced by the selected playback rate.
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    }

    timer.set_duration(std::time::Duration::from_secs_f32(
        1.0 / resources.playback.current_speed().frames_per_second,
    ));

    timer.tick(resources.time.delta());

    if key_repeat_triggered(
        &mut key_repeat.forward,
        &resources.keys,
        KeyCode::KeyD,
        resources.time.delta(),
    ) {
        step_forward(resources.viewer.as_mut(), resources.playback.as_mut());
    } else if key_repeat_triggered(
        &mut key_repeat.backward,
        &resources.keys,
        KeyCode::KeyA,
        resources.time.delta(),
    ) {
        let _ = resources.viewer.step_frame(-1);
    }

    if !resources.playback.is_playing {
        return;
    }

    for _ in 0..timer.times_finished_this_tick() {
        if !step_forward(resources.viewer.as_mut(), resources.playback.as_mut()) {
            if !resources.viewer.follow_tail {
                resources.playback.is_playing = false;
            }
            break;
        }
    }
}

fn key_repeat_triggered(
    repeat: &mut KeyRepeat,
    keys: &ButtonInput<KeyCode>,
    key: KeyCode,
    delta: std::time::Duration,
) -> bool {
    if keys.just_pressed(key) {
        repeat.timer = Some(Timer::from_seconds(0.25, TimerMode::Once));
        repeat.repeating = false;
        return true;
    }

    if !keys.pressed(key) {
        repeat.timer = None;
        repeat.repeating = false;
        return false;
    }

    let Some(timer) = repeat.timer.as_mut() else {
        repeat.timer = Some(Timer::from_seconds(0.25, TimerMode::Once));
        return false;
    };

    timer.tick(delta);
    if !timer.just_finished() {
        return false;
    }

    if !repeat.repeating {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
        repeat.repeating = true;
    }
    true
}

pub fn step_forward(viewer: &mut ViewerState, playback: &mut PlaybackState) -> bool {
    let advanced = viewer.step_frame(1);
    if !advanced && !viewer.follow_tail {
        playback.is_playing = false;
    }
    advanced
}

#[cfg(test)]
mod tests {
    use super::{despawn_current_frame, navigate_frames, step_forward};
    use crate::components::{
        FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameMeasurementCue,
        FrameSelectionHighlight,
    };
    use crate::ui::PlaybackState;
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
        ) = system_state.get_mut(&mut world);
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
        app.insert_resource(PlaybackState::default());
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
        app.insert_resource(PlaybackState::default());
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
        app.insert_resource(PlaybackState::default());
        app.add_systems(Update, navigate_frames);

        app.update();

        let viewer = app.world().resource::<ViewerState>();
        assert_eq!(viewer.supercell.repeats, [1, 0, 0]);
        assert!(!viewer.supercell.ghost_repeated_images);
    }

    #[test]
    fn step_forward_pauses_playback_at_end_without_follow_tail() {
        let mut viewer = sample_viewer();
        viewer.current = 1;
        let mut playback = PlaybackState {
            is_playing: true,
            ..Default::default()
        };

        let advanced = step_forward(&mut viewer, &mut playback);

        assert!(!advanced);
        assert!(!playback.is_playing);
    }

    #[test]
    fn navigate_frames_advances_playback_when_running() {
        let mut app = App::new();
        let keys = ButtonInput::<KeyCode>::default();
        let logical_keys = ButtonInput::<Key>::default();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(0.25));

        app.insert_resource(keys);
        app.insert_resource(logical_keys);
        app.insert_resource(time);
        app.insert_resource(sample_viewer());
        app.insert_resource(PlaybackState {
            is_playing: true,
            speed_index: 1,
            ..Default::default()
        });
        app.add_systems(Update, navigate_frames);

        app.update();

        assert_eq!(app.world().resource::<ViewerState>().current, 1);
    }

    #[test]
    fn navigate_frames_repeats_after_hold_delay() {
        let mut app = App::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyD);
        let logical_keys = ButtonInput::<Key>::default();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(0.30));

        app.insert_resource(keys.clone());
        app.insert_resource(logical_keys.clone());
        app.insert_resource(time);
        app.insert_resource(sample_viewer());
        app.insert_resource(PlaybackState::default());
        app.add_systems(Update, navigate_frames);

        app.update();
        assert_eq!(app.world().resource::<ViewerState>().current, 1);

        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(0.30));
        app.insert_resource(time);
        app.insert_resource(keys);
        app.insert_resource(logical_keys);
        app.update();

        assert_eq!(app.world().resource::<ViewerState>().current, 1);
    }
}
