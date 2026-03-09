use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::components::{FrameAtom, FrameAxis, FrameCell};
use crate::viewer::ViewerState;

#[derive(SystemParam)]
pub struct RenderResources<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    viewer: ResMut<'w, ViewerState>,
}

pub fn despawn_current_frame(
    commands: &mut Commands,
    atoms: Query<Entity, With<FrameAtom>>,
    cells: Query<Entity, With<FrameCell>>,
    axes: Query<Entity, With<FrameAxis>>,
) {
    for entity in atoms.iter() {
        commands.entity(entity).despawn();
    }
    for entity in cells.iter() {
        commands.entity(entity).despawn();
    }
    for entity in axes.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn navigate_frames(mut timer: Local<Timer>, mut resources: RenderResources) {
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
