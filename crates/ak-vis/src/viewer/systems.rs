use crate::render::{render_atoms, render_axis, render_bonds, render_cell, render_faces};
use crate::visuals::{BondVisual, FaceVisual};
use crate::{JMOL, convert_axis, convert_cell, convert_structure, structure_position_to_world};

use crate::components::{FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, MainSceneCamera};
use crate::viewer::ViewerConfig;
use crate::viewer::controls::default_camera_state;
use crate::viewer::controls::despawn_current_frame;
use crate::viewer::runtime::{CommandReceiver, MainCameraRenderTarget};
use crate::viewer::{
    AtomColorRule, BallAndStickStyle, BondList, BondScope, CameraState, RenderStyle, ViewerState,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::collections::HashMap;

fn scalar_colors_for_rule(
    viewer: &ViewerState,
    rule: &AtomColorRule,
) -> Option<Vec<Option<Color>>> {
    let values = viewer
        .atom_scalars
        .get(&rule.name)
        .and_then(|frames| frames.get(viewer.current))
        .and_then(|values| values.as_ref())?;

    if values.len() != viewer.traj.view(viewer.current).positions.len() {
        return None;
    }

    let finite_values: Vec<f32> = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    if finite_values.is_empty() {
        return None;
    }

    let inferred_min = finite_values.iter().copied().fold(f32::INFINITY, f32::min);
    let inferred_max = finite_values
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);
    let min = rule.min.unwrap_or(inferred_min);
    let max = rule.max.unwrap_or(inferred_max);
    let span = (max - min).max(f32::EPSILON);

    Some(
        values
            .iter()
            .map(|value| {
                if value.is_finite() {
                    Some(rule.palette.color((value - min) / span))
                } else {
                    None
                }
            })
            .collect(),
    )
}

fn scalar_colors_for_current_frame(viewer: &ViewerState) -> Option<Vec<Option<Color>>> {
    if viewer.atom_color_rules.is_empty() {
        return None;
    }

    let atom_count = viewer.traj.view(viewer.current).positions.len();
    let mut layered_colors = vec![None; atom_count];
    let mut applied_any = false;

    for rule in &viewer.atom_color_rules {
        let Some(rule_colors) = scalar_colors_for_rule(viewer, rule) else {
            continue;
        };
        for (slot, color) in layered_colors.iter_mut().zip(rule_colors) {
            if color.is_some() {
                *slot = color;
                applied_any = true;
            }
        }
    }

    applied_any.then_some(layered_colors)
}

fn resolved_atom_styles_for_current_frame(
    viewer: &ViewerState,
) -> Option<Vec<Option<BallAndStickStyle>>> {
    let atom_count = viewer.traj.view(viewer.current).positions.len();
    let mut styles = vec![None; atom_count];
    let mut applied_any = false;

    for rule in viewer
        .render_style_rules
        .iter()
        .filter(|rule| rule.frame_index == viewer.current)
    {
        if let RenderStyle::BallAndStick(style) = rule.style {
            for (slot, selected) in styles.iter_mut().zip(rule.selection.iter().copied()) {
                if selected {
                    *slot = Some(style);
                    applied_any = true;
                }
            }
        }
    }

    applied_any.then_some(styles)
}

fn resolved_bond_styles_for_current_frame(
    viewer: &ViewerState,
    bonds: &BondList,
) -> Option<HashMap<(usize, usize), BallAndStickStyle>> {
    let mut styles = HashMap::new();

    for rule in viewer
        .render_style_rules
        .iter()
        .filter(|rule| rule.frame_index == viewer.current)
    {
        if let RenderStyle::BallAndStick(style) = rule.style {
            for &(i, j) in bonds.iter() {
                if i >= rule.selection.len() || j >= rule.selection.len() {
                    continue;
                }
                let include = match style.bond_scope {
                    BondScope::BothSelected => rule.selection[i] && rule.selection[j],
                    BondScope::TouchSelection => rule.selection[i] || rule.selection[j],
                };
                if include {
                    styles.insert((i, j), style);
                }
            }
        }
    }

    (!styles.is_empty()).then_some(styles)
}

fn bond_visuals_for_current_frame(viewer: &ViewerState) -> Vec<BondVisual> {
    let Some(bonds) = viewer.bonds.get(viewer.current) else {
        return Vec::new();
    };
    let Some(styles) = resolved_bond_styles_for_current_frame(viewer, bonds) else {
        return Vec::new();
    };

    let view = viewer.traj.view(viewer.current);
    let mut visuals = Vec::new();
    for &(i, j) in bonds.iter() {
        let Some(style) = styles.get(&(i, j)).copied() else {
            continue;
        };
        let start = structure_position_to_world(view.positions[i]);
        let end = structure_position_to_world(view.positions[j]);
        visuals.push(BondVisual {
            start,
            end,
            color: style.bond_color(),
            radius: style.bond_radius,
        });
    }
    visuals
}

fn face_visuals_for_current_frame(viewer: &ViewerState) -> Vec<FaceVisual> {
    let Some(faces) = viewer.faces.get(viewer.current) else {
        return Vec::new();
    };

    let view = viewer.traj.view(viewer.current);
    let mut visuals = Vec::new();
    for face in faces.iter() {
        let Some(vertices) = face
            .atoms
            .iter()
            .map(|&index| view.positions.get(index).copied())
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        visuals.push(FaceVisual {
            vertices: vertices
                .into_iter()
                .map(structure_position_to_world)
                .collect(),
            color: face.clone().color(),
        });
    }
    visuals
}

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
    let scalar_colors = scalar_colors_for_current_frame(viewer);
    let ball_and_stick_styles = resolved_atom_styles_for_current_frame(viewer);
    let atom_visuals = convert_structure(&view, &JMOL)
        .into_iter()
        .enumerate()
        .map(|(index, mut visual)| {
            if let Some(colors) = scalar_colors.as_ref()
                && let Some(color) = colors[index]
            {
                visual.color = color;
            }
            if let Some(styles) = ball_and_stick_styles.as_ref()
                && let Some(style) = styles[index]
            {
                visual.radius *= style.atom_scale;
            }
            visual
        })
        .collect();
    render_atoms(
        atom_visuals,
        commands,
        materials,
        meshes,
        config.render.ico_subdiv,
    );

    let bond_visuals = bond_visuals_for_current_frame(viewer);
    if !bond_visuals.is_empty() {
        render_bonds(bond_visuals, commands, materials, meshes);
    }

    let face_visuals = face_visuals_for_current_frame(viewer);
    if !face_visuals.is_empty() {
        render_faces(face_visuals, commands, materials, meshes);
    }

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

pub fn setup_camera(
    mut commands: Commands,
    camera: Res<CameraState>,
    config: Res<ViewerConfig>,
    render_target: Option<Res<MainCameraRenderTarget>>,
) {
    if camera.radius <= 0.0 {
        return;
    }

    let is_headless = render_target.is_some();
    let mut camera = commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        PanOrbitCamera {
            yaw: Some(camera.yaw),
            pitch: Some(camera.pitch),
            radius: Some(camera.radius),
            focus: camera.focus,
            axis: [Vec3::X, Vec3::Y, Vec3::Z],
            orbit_smoothness: if is_headless { 0.0 } else { 0.1 },
            pan_smoothness: if is_headless { 0.0 } else { 0.02 },
            zoom_smoothness: if is_headless { 0.0 } else { 0.1 },
            ..default()
        },
    ));
    if let Some(render_target) = render_target {
        camera.insert(render_target.0.clone());
    }
    camera.insert(MainSceneCamera);
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
    mut camera: ResMut<CameraState>,
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
                camera.apply_command(viewer.as_ref(), &command);
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
    bonds: Query<'w, 's, Entity, With<FrameBond>>,
    faces: Query<'w, 's, Entity, With<FrameFace>>,
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
    mut camera: ResMut<CameraState>,
    config: Res<ViewerConfig>,
    queries: RenderFrameQueries,
) {
    if !viewer.needs_render || !viewer.has_frames() {
        return;
    }

    despawn_current_frame(
        &mut commands,
        queries.atoms,
        queries.cells,
        queries.axes,
        queries.bonds,
        queries.faces,
    );
    render_frame(
        &mut commands,
        &mut assets.meshes,
        &mut assets.materials,
        viewer.as_mut(),
        config.as_ref(),
    );

    if viewer.needs_camera_reset {
        *camera = default_camera_state(viewer.as_ref());
        viewer.needs_camera_reset = false;
    }
}

pub fn advance_camera_motion(time: Res<Time>, mut camera: ResMut<CameraState>) {
    camera.tick_motion(time.delta_secs());
}

pub fn apply_camera_state(
    mut camera_state: ResMut<CameraState>,
    mut camera_query: Query<&mut PanOrbitCamera>,
) {
    if !camera_state.needs_apply {
        return;
    }

    let Ok(mut orbit) = camera_query.single_mut() else {
        return;
    };

    orbit.target_focus = camera_state.focus;
    orbit.target_radius = camera_state.radius;
    orbit.target_yaw = camera_state.yaw;
    orbit.target_pitch = camera_state.pitch;
    orbit.force_update = true;
    camera_state.needs_apply = false;
}
