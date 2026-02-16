use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::{FrameAtom, FrameAxis, FrameCell};
use crate::render::{render_atoms, render_axis, render_cell};
use crate::viewer::ViewerConfig;
use crate::viewer::app::ViewerTrajectory;
use crate::{JMOL, convert_axis, convert_cell, convert_structure};
use crate::viewer::controls::utils::default_radius_focus;

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
    keys: Res<ButtonInput<KeyCode>>,
    mut viewer: ResMut<ViewerTrajectory>,
    mut commands: Commands,
    // Queries for despawning
    atoms: Query<Entity, With<FrameAtom>>,
    cells: Query<Entity, With<FrameCell>>,
    axes: Query<Entity, With<FrameAxis>>,
    // Resources for rendering
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<ViewerConfig>,
    // Resources for camera setting:
    cam_query: Query<&mut PanOrbitCamera>,

) {
    let mut changed = false;

    if keys.just_pressed(KeyCode::KeyD) {
        if viewer.current < viewer.traj.len() - 1 {
            viewer.current += 1;
            changed = true;
        }
    }

    if keys.just_pressed(KeyCode::KeyA) {
        if viewer.current > 0 {
            viewer.current -= 1;
            changed = true;
        }
    }

    if changed {
        // Despawn old frame
        despawn_current_frame(&mut commands, atoms, cells, axes);

        // Render new frame (same logic as render_current_frame)
        let view = viewer.traj.view(viewer.current);
        let atom_visuals = convert_structure(&view, &JMOL);
        render_atoms(atom_visuals, &mut commands, &mut materials, &mut meshes);

        if config.render.show_cell {
            let cell_visuals = convert_cell(&view, config.color.cell_color);
            render_cell(cell_visuals, &mut commands, &mut materials, &mut meshes);
        }

        if config.render.show_axes {
            let axis_visuals = convert_axis(&view);
            render_axis(axis_visuals, &mut commands, &mut materials, &mut meshes);
        }

        default_radius_focus(viewer.into(), cam_query);

    }
}
