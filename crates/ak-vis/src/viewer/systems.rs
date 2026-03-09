use crate::render::{render_atoms, render_axis, render_cell};
use crate::{JMOL, convert_axis, convert_cell, convert_structure};
use std::f32::consts::FRAC_PI_2;

use crate::components::{FrameAtom, FrameAxis, FrameCell};
use crate::viewer::ViewerConfig;
use crate::viewer::ViewerState;
use crate::viewer::app::CommandReceiver;
use crate::viewer::controls::default_radius_focus;
use crate::viewer::controls::despawn_current_frame;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

fn render_frame(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    viewer: &mut ViewerState,
    config: &ViewerConfig,
) {
    if !viewer.has_frames() {
        return;
    }

    let view = viewer.traj.view(viewer.current);
    let atom_visuals = convert_structure(&view, &JMOL);
    render_atoms(
        atom_visuals,
        commands,
        materials,
        meshes,
        config.render.ico_subdiv,
    );

    if config.render.show_cell {
        let cell_visuals = convert_cell(&view, config.color.cell_color);
        render_cell(cell_visuals, commands, materials, meshes);
    }

    if config.render.show_axes {
        let axis_visuals = convert_axis(&view);
        render_axis(axis_visuals, commands, materials, meshes);
    }

    viewer.needs_render = false;
}

pub fn render_current_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut viewer: ResMut<ViewerState>,
    config: Res<ViewerConfig>,
) {
    render_frame(
        &mut commands,
        &mut meshes,
        &mut materials,
        viewer.as_mut(),
        config.as_ref(),
    );
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

pub fn setup_camera(mut commands: Commands, viewer: Res<ViewerState>, config: Res<ViewerConfig>) {
    if !viewer.has_frames() {
        return;
    }

    let view = viewer.traj.view(viewer.current);

    let cell_midpoint = Vec3::from_slice(view.cell.reduced(0.5, 0.5, 0.5).cast::<f32>().as_slice());

    let max_cell_length = [view.cell.a(), view.cell.b(), view.cell.c()]
        .iter()
        .fold(0.0_f64, |acc, v| acc.max(v.norm())) as f32;

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
    let Ok((cam_gt, orbit)) = cam_q.single() else {
        return;
    };
    let Ok(mut light_transform) = light_q.single_mut() else {
        return;
    };

    let cam_pos = cam_gt.translation();
    let to_focus = (orbit.focus - cam_pos).normalize_or_zero();

    // Directional light shines along its -Z axis
    light_transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, to_focus);
}

pub fn apply_viewer_commands(
    mut viewer: ResMut<ViewerState>,
    receiver: Res<CommandReceiver>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    let Some(receiver) = receiver.0.as_ref() else {
        return;
    };

    let Ok(receiver) = receiver.lock() else {
        return;
    };

    loop {
        match receiver.try_recv() {
            Ok(command) => {
                let outcome = viewer.apply_command(command);
                if outcome.should_close {
                    app_exit_events.write(AppExit::Success);
                    break;
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
        }
    }
}

#[derive(SystemParam)]
pub struct RenderFrameQueries<'w, 's> {
    atoms: Query<'w, 's, Entity, With<FrameAtom>>,
    cells: Query<'w, 's, Entity, With<FrameCell>>,
    axes: Query<'w, 's, Entity, With<FrameAxis>>,
    camera: Query<'w, 's, &'static mut PanOrbitCamera>,
}

#[derive(SystemParam)]
pub struct RenderFrameAssets<'w> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
}

pub fn rerender_if_dirty(
    mut commands: Commands,
    mut assets: RenderFrameAssets,
    mut viewer: ResMut<ViewerState>,
    config: Res<ViewerConfig>,
    queries: RenderFrameQueries,
) {
    if !viewer.needs_render || !viewer.has_frames() {
        return;
    }

    despawn_current_frame(&mut commands, queries.atoms, queries.cells, queries.axes);
    render_frame(
        &mut commands,
        &mut assets.meshes,
        &mut assets.materials,
        viewer.as_mut(),
        config.as_ref(),
    );

    if viewer.needs_camera_reset {
        default_radius_focus(viewer.as_ref(), queries.camera);
        viewer.needs_camera_reset = false;
    }
}
