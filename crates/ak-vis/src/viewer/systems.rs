use crate::render::{render_atoms, render_axis, render_cell};
use crate::{JMOL, convert_axis, convert_cell, convert_structure};
use std::f32::consts::FRAC_PI_2;

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

    if config.render.show_cell {
        let cell_visuals = convert_cell(&view, config.color.cell_color);
        render_cell(cell_visuals, &mut commands, &mut materials, &mut meshes);
    }

    if config.render.show_axes {
        let axis_visuals = convert_axis(&view);
        render_axis(axis_visuals, &mut commands, &mut materials, &mut meshes);
    }
}

pub fn setup_lighting(
    mut commands: Commands,
    mut ambient_light: ResMut<GlobalAmbientLight>,
    config: Res<ViewerConfig>,
) {
    ambient_light.brightness = config.lighting.ambient_brightness;

    // KEY light (main): above + to the side + from front
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.key_illuminance,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            -0.8, // yaw
            -0.6, // pitch
            0.0,
        )),
    ));

    // FILL light (soft): opposite side, weaker, no shadows
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.fill_illuminance,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 1.6, -0.2, 0.0)),
    ));

    // RIM / BACK light: behind to pop silhouettes a bit
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.back_illuminance,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -2.6, -0.3, 0.0)),
    ));
}

pub fn setup_camera(
    mut commands: Commands,
    viewer: Res<ViewerTrajectory>,
    config: Res<ViewerConfig>,
) {
    // Setup camera

    let view = viewer.traj.view(viewer.current);

    let cell_midpoint = Vec3::from_array(view.cell.reduced(0.5, 0.5, 0.5).map(|f| f as f32));

    let max_cell_length = [view.cell.a(), view.cell.b(), view.cell.c()]
        .iter()
        .map(|vec| vec.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as f32;

    let mut camera = commands.spawn(PanOrbitCamera {
        yaw: Some(-FRAC_PI_2),
        pitch: Some(0.0),
        radius: Some(2.5 * max_cell_length),
        focus: cell_midpoint,
        axis: [Vec3::X, Vec3::Y, Vec3::Z],
        ..default()
    });
    if config.lighting.enable_fog {
        camera.insert(DistanceFog {
            color: config.color.background,
            directional_light_color: Color::WHITE,
            directional_light_exponent: 5.0,
            falloff: FogFalloff::Exponential { density: 0.0015 },
        });
    }
}

#[derive(Component)]
pub struct CameraLight;

pub fn setup_camera_light(mut commands: Commands, config: Res<ViewerConfig>) {
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.camera_illuminance,
            shadows_enabled: true,
            ..default()
        },
        CameraLight,
    ));
}

pub fn update_camera_light(
    cam_q: Query<(&GlobalTransform, &PanOrbitCamera)>,
    mut light_q: Query<&mut Transform, With<CameraLight>>,
) {
    let (cam_gt, orbit) = cam_q.single().unwrap();
    let mut light_transform = light_q.single_mut().unwrap();

    let cam_pos = cam_gt.translation();
    let to_focus = (orbit.focus - cam_pos).normalize_or_zero();

    // Directional light shines along its -Z axis
    light_transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, to_focus);
}
