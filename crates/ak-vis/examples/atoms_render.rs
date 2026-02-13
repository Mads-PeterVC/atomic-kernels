//! A simple 3D scene with light shining over a cube sitting on a plane.

use ak_core::Structure;
use ak_vis::{JMOL, convert_cell, convert_structure, convert_axis};

use ak_vis::render::{render_cell, render_atoms, render_axis};

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.98, 0.98, 0.98)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup, setup_ambient_light))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load structure from disk
    let structure = Structure::from_xyz_file(
        "/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/optimized_structure.xyz",
    );

    // Create and render atoms
    let view = structure.view();
    let atom_visuals = convert_structure(&view, &JMOL);
    let cell_visuals = convert_cell(&view);
    let axis_visuals = convert_axis(&view);



    render_atoms(atom_visuals, &mut commands, &mut materials, &mut meshes);
    render_cell(cell_visuals, &mut commands, &mut materials, &mut meshes);
    render_axis(axis_visuals, &mut commands, &mut materials, &mut meshes);

    // Setup lights
    let cardinals = [Vec3::Y, -Vec3::Y, Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z];
    // let cardinals = [-Vec3::Y];

    for dir in cardinals {
        commands.spawn((
            DirectionalLight {
                illuminance: 1_000.0,
                ..default()
            },
            Transform::from_rotation(Quat::from_rotation_arc(Vec3::Y, dir)),
        ));
    }

    // Setup camera

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

fn setup_ambient_light(mut ambient_light: ResMut<GlobalAmbientLight>) {
    ambient_light.brightness = 1000.0;
}
