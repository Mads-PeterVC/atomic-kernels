use crate::render::{render_atoms, render_axis, render_cell};
use crate::{JMOL, convert_axis, convert_cell, convert_structure};

use crate::viewer::ViewerConfig;
use crate::viewer::app::ViewerTrajectory;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn render_current_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    viewer: Res<ViewerTrajectory>,
    config: Res<ViewerConfig>,
) {
    let view = viewer.traj.view(viewer.current);

    // Create and render atoms
    let atom_visuals = convert_structure(&view, &JMOL);
    render_atoms(atom_visuals, &mut commands, &mut materials, &mut meshes);

    if config.show_cell {
        let cell_visuals = convert_cell(&view);
        render_cell(cell_visuals, &mut commands, &mut materials, &mut meshes);
    }

    if config.show_axes {
        let axis_visuals = convert_axis(&view);
        render_axis(axis_visuals, &mut commands, &mut materials, &mut meshes);
    }
}

pub fn setup_lighting(
    mut commands: Commands,
    mut ambient_light: ResMut<GlobalAmbientLight>,
    config: Res<ViewerConfig>,
) {
    // Ambient light
    ambient_light.brightness = config.ambient_brightness;

    // Directional lights
    let cardinals = [Vec3::Y, -Vec3::Y, Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z];

    for dir in cardinals {
        commands.spawn((
            DirectionalLight {
                illuminance: config.directional_illuminance,
                ..default()
            },
            Transform::from_rotation(Quat::from_rotation_arc(Vec3::Y, dir)),
        ));
    }
}

pub fn setup_camera(mut commands: Commands, viewer: Res<ViewerTrajectory>) {
    // Setup camera

    let view = viewer.traj.view(viewer.current);

    let cell_midpoint = Vec3::from_array([
        0.5 * (view.cell.m[0][0] + view.cell.m[1][0] + view.cell.m[2][0]) as f32,
        0.5 * (view.cell.m[0][1] + view.cell.m[1][1] + view.cell.m[2][1]) as f32,
        0.5 * (view.cell.m[0][2] + view.cell.m[1][2] + view.cell.m[2][2]) as f32,
    ]);

    commands.spawn((
        Transform::from_translation(Vec3::new(-10.0, -10.0, cell_midpoint.z)),
        PanOrbitCamera {
            focus: cell_midpoint,
            ..default()
        },
    ));
}
