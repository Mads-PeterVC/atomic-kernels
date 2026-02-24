use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::{FrameAtom, FrameAxis, FrameCell};
use crate::render::{render_atoms, render_axis, render_cell};
use crate::viewer::ViewerConfig;
use crate::viewer::app::ViewerTrajectory;
use crate::viewer::controls::utils::default_radius_focus;
use crate::{JMOL, convert_axis, convert_cell, convert_structure};

#[derive(SystemParam)]
pub struct FrameQueries<'w, 's> {
    atoms: Query<'w, 's, Entity, With<FrameAtom>>,
    cells: Query<'w, 's, Entity, With<FrameCell>>,
    axes: Query<'w, 's, Entity, With<FrameAxis>>,
    camera: Query<'w, 's, &'static mut PanOrbitCamera>,
}

#[derive(SystemParam)]
pub struct RenderResources<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    viewer: ResMut<'w, ViewerTrajectory>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    config: Res<'w, ViewerConfig>,
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

pub fn navigate_frames(
    mut commands: Commands,
    mut timer: Local<Timer>,
    queries: FrameQueries,
    mut resources: RenderResources,
) {
    // Initialize timer on first run (0.1 seconds = 10 frames per second)
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    }

    timer.tick(resources.time.delta());

    // Only advance frame when timer finishes
    if !timer.just_finished() {
        return;
    }

    let mut changed = false;

    if resources.keys.pressed(KeyCode::KeyD)
        && resources.viewer.current < resources.viewer.traj.len() - 1
    {
        resources.viewer.current += 1;
        changed = true;
    }

    if resources.keys.pressed(KeyCode::KeyA) && resources.viewer.current > 0 {
        resources.viewer.current -= 1;
        changed = true;
    }

    if changed {
        // Despawn old frame
        despawn_current_frame(&mut commands, queries.atoms, queries.cells, queries.axes);

        // Render new frame (same logic as render_current_frame)
        let view = resources.viewer.traj.view(resources.viewer.current);
        let atom_visuals = convert_structure(&view, &JMOL);
        render_atoms(
            atom_visuals,
            &mut commands,
            &mut resources.materials,
            &mut resources.meshes,
            resources.config.render.ico_subdiv,
        );

        if resources.config.render.show_cell {
            let cell_visuals = convert_cell(&view, resources.config.color.cell_color);
            render_cell(
                cell_visuals,
                &mut commands,
                &mut resources.materials,
                &mut resources.meshes,
            );
        }

        if resources.config.render.show_axes {
            let axis_visuals = convert_axis(&view);
            render_axis(
                axis_visuals,
                &mut commands,
                &mut resources.materials,
                &mut resources.meshes,
            );
        }

        // Only check for cell changes if there's a previous frame
        if resources.viewer.current > 0 {
            let previous_cell = resources
                .viewer
                .traj
                .view(resources.viewer.current - 1)
                .cell;
            let current_cell = resources.viewer.traj.view(resources.viewer.current).cell;

            if !current_cell.eq(&previous_cell) {
                default_radius_focus(resources.viewer.into(), queries.camera);
            }
        }
    }
}
