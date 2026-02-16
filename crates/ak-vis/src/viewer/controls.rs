use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Capturing, Screenshot},
    window::{CursorIcon, SystemCursorIcon},
};
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::FRAC_PI_2;

use crate::viewer::app::ViewerTrajectory;
use crate::components::{FrameAtom, FrameCell, FrameAxis};
use crate::viewer::ViewerConfig;
use crate::render::{render_atoms, render_axis, render_cell};
use crate::{JMOL, convert_axis, convert_cell, convert_structure};

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
    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
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

pub fn screenshot_on_spacebar(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

pub fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    window: Single<Entity, With<Window>>,
) {
    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(*window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
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
       }
   }
