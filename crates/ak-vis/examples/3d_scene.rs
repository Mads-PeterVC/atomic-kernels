//! A simple 3D scene with light shining over a cube sitting on a plane.

use ak_core::io::read_xyz;
use ak_core::{PERIODIC_TABLE, StructureView};
use ak_vis::{AtomVisual, JMOL};
use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.98, 0.98, 0.98)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn make_visual(view: StructureView) -> Vec<AtomVisual> {
    let mut visuals = Vec::new();
    for i in 0..view.positions.len() {
        let radius: f32 = PERIODIC_TABLE.get(view.numbers[i]).covalent_radius as f32;
        let color = JMOL.get(view.numbers[i]);
        let atom_visual = AtomVisual::new(view.positions[i], color, radius);
        visuals.push(atom_visual);
    }
    visuals
}

fn render_atoms(
    visuals: Vec<AtomVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    for atom in visuals.iter() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(atom.radius))),
            MeshMaterial3d(materials.add(atom.color)),
            Transform::from_xyz(atom.x(), atom.y(), atom.z()),
        ));
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let file = File::open("/Users/au616397/Repositories/atomic-kernels/crates/ak-vis/examples/xyz/cluster_auag_ico_4_core_shell.xyz").unwrap();
    let reader = BufReader::new(file);
    let structure = read_xyz(reader);

    let view = structure.view();
    let visuals = make_visual(view);
    render_atoms(visuals, &mut commands, &mut materials, &mut meshes);

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)),
    ));

    // commands.spawn(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 10.0,
    //     affects_lightmapped_meshes: false,
    // });

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

