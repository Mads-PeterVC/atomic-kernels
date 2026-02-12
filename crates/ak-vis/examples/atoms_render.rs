//! A simple 3D scene with light shining over a cube sitting on a plane.

use ak_core::Structure;
use ak_vis::{JMOL, convert_structure, render_atoms};
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
        "/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/cluster_auag_ico_4_core_shell.xyz",
    );

    // Create and render atoms
    let view = structure.view();
    let visuals = convert_structure(view, &JMOL);
    render_atoms(visuals, &mut commands, &mut materials, &mut meshes);

    // let weird_color = Color::srgb_u8(0, 0, 0);
    // commands.spawn((Mesh3d(meshes.add(Cylinder::new(0.1, 100.0))),
    //         MeshMaterial3d(materials.add(weird_color)),
    //         Transform::from_xyz(0.0, 0.0, 0.0)
    //     ));

    // Setup lights
    let cardinals = [Vec3::Y, -Vec3::Y, Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z];

    for dir in cardinals {
        commands.spawn((
            DirectionalLight {
                illuminance: 2_000.0,
                ..default()
            },
            Transform::from_rotation(Quat::from_rotation_arc(Vec3::Y, dir)),
        ));
    }

    // Setup camera
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

fn setup_ambient_light(mut ambient_light: ResMut<GlobalAmbientLight>) {
    ambient_light.brightness = 1000.0;
}
