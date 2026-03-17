use crate::render::{render_atoms, render_axis, render_bonds, render_cell, render_faces};
use crate::visuals::{BondVisual, FaceVisual};
use crate::{JMOL, convert_axis, convert_cell, convert_structure, structure_position_to_world};

use crate::components::{
    AtomIndex, FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameSelectionHighlight,
    MainSceneCamera, MarqueeSelectionOverlay,
};
use crate::viewer::ViewerConfig;
use crate::viewer::controls::default_camera_state;
use crate::viewer::controls::despawn_current_frame;
use crate::viewer::runtime::{CommandReceiver, MainCameraRenderTarget, SharedViewerSnapshot};
use crate::viewer::{
    AtomColorRule, BallAndStickStyle, BondList, BondScope, CameraState, RenderStyle, ViewerState,
};
use bevy::ecs::system::SystemParam;
use bevy::picking::prelude::{Click, Pickable, Pointer, PointerButton};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::collections::HashMap;

const MARQUEE_DRAG_THRESHOLD: f32 = 6.0;

#[derive(Resource, Debug, Default, Clone)]
pub struct MarqueeSelectionState {
    drag_start: Option<Vec2>,
    drag_current: Option<Vec2>,
    drag_active: bool,
    suppress_click_once: bool,
}

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

fn selection_highlight_visuals_for_current_frame(viewer: &ViewerState) -> Vec<crate::AtomVisual> {
    let selection = viewer.current_selection();
    if selection.is_empty() || !selection.iter().any(|selected| *selected) {
        return Vec::new();
    }

    convert_structure(&viewer.traj.view(viewer.current), &JMOL)
        .into_iter()
        .filter_map(|mut visual| {
            if !selection.get(visual.atom_index).copied().unwrap_or(false) {
                return None;
            }
            visual.radius = selection_highlight_radius(visual.radius);
            visual.color = selection_highlight_color();
            Some(visual)
        })
        .collect()
}

fn shift_held(keys: &ButtonInput<KeyCode>) -> bool {
    keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
}

fn viewport_contains(camera: &Camera, point: Vec2) -> bool {
    camera
        .logical_viewport_rect()
        .map(|rect| rect.contains(point))
        .unwrap_or(true)
}

fn marquee_drag_exceeded(start: Vec2, current: Vec2) -> bool {
    start.distance(current) >= MARQUEE_DRAG_THRESHOLD
}

fn marquee_rect(start: Vec2, current: Vec2) -> Rect {
    Rect::from_corners(start, current)
}

fn selection_mask_for_projected_positions(
    atom_count: usize,
    projected_positions: impl IntoIterator<Item = (usize, Vec2)>,
    rect: Rect,
) -> Vec<bool> {
    let mut mask = vec![false; atom_count];
    for (index, viewport_position) in projected_positions {
        if index < atom_count && rect.contains(viewport_position) {
            mask[index] = true;
        }
    }
    mask
}

fn projected_selection_for_rect(
    viewer: &ViewerState,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    rect: Rect,
) -> Vec<bool> {
    let view = viewer.traj.view(viewer.current);
    let projected = view
        .positions
        .iter()
        .enumerate()
        .filter_map(|(index, position)| {
            camera
                .world_to_viewport(camera_transform, structure_position_to_world(*position))
                .ok()
                .map(|viewport_position| (index, viewport_position))
        });
    selection_mask_for_projected_positions(view.positions.len(), projected, rect)
}

fn selection_highlight_radius(atom_radius: f32) -> f32 {
    atom_radius + 0.08 + atom_radius * 0.12
}

fn selection_highlight_color() -> Color {
    Color::srgba(1.0, 0.55, 0.08, 0.42)
}

fn render_selection_highlights(
    visuals: Vec<crate::AtomVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    quality: u32,
) {
    for visual in visuals {
        let mesh = meshes.add(
            Sphere::new(visual.radius)
                .mesh()
                .ico(quality)
                .expect("Failed to create selection highlight sphere"),
        );
        let material = materials.add(StandardMaterial {
            base_color: visual.color,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(visual.x(), visual.y(), visual.z())),
            FrameSelectionHighlight,
            Pickable::IGNORE,
        ));
    }
}

impl MarqueeSelectionState {
    fn is_tracking(&self) -> bool {
        self.drag_start.is_some()
    }

    fn begin(&mut self, cursor: Vec2) {
        self.drag_start = Some(cursor);
        self.drag_current = Some(cursor);
        self.drag_active = false;
    }

    fn update(&mut self, cursor: Vec2) {
        let Some(start) = self.drag_start else {
            return;
        };
        self.drag_current = Some(cursor);
        if !self.drag_active && marquee_drag_exceeded(start, cursor) {
            self.drag_active = true;
        }
    }

    fn active_rect(&self) -> Option<Rect> {
        if !self.drag_active {
            return None;
        }
        Some(marquee_rect(self.drag_start?, self.drag_current?))
    }

    fn finish(&mut self) -> Option<Rect> {
        let rect = self.active_rect();
        self.drag_start = None;
        self.drag_current = None;
        self.drag_active = false;
        if rect.is_some() {
            self.suppress_click_once = true;
        }
        rect
    }

    fn take_click_suppression(&mut self) -> bool {
        std::mem::take(&mut self.suppress_click_once)
    }
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

    let selection_highlights = selection_highlight_visuals_for_current_frame(viewer);
    if !selection_highlights.is_empty() {
        render_selection_highlights(
            selection_highlights,
            commands,
            materials,
            meshes,
            config.render.ico_subdiv,
        );
    }

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

pub fn setup_marquee_overlay(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(0.0),
            top: px(0.0),
            width: px(0.0),
            height: px(0.0),
            border: UiRect::all(px(1.5)),
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.55, 0.08, 0.12)),
        BorderColor::all(Color::srgba(1.0, 0.7, 0.2, 0.85)),
        Visibility::Hidden,
        ZIndex(50),
        MarqueeSelectionOverlay,
    ));
}

pub fn handle_marquee_selection(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut camera_query: Query<
        (&Camera, &GlobalTransform, &mut PanOrbitCamera),
        With<MainSceneCamera>,
    >,
    mut marquee: ResMut<MarqueeSelectionState>,
    mut viewer: ResMut<ViewerState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform, mut pan_orbit)) = camera_query.single_mut() else {
        return;
    };
    let cursor_position = window.cursor_position();

    if mouse_buttons.just_pressed(MouseButton::Left)
        && shift_held(&keys)
        && let Some(cursor_position) = cursor_position
        && viewport_contains(camera, cursor_position)
    {
        marquee.begin(cursor_position);
        pan_orbit.enabled = false;
    }

    if !marquee.is_tracking() {
        return;
    }

    if mouse_buttons.pressed(MouseButton::Left)
        && let Some(cursor_position) = cursor_position
    {
        marquee.update(cursor_position);
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        let rect = marquee.finish();
        pan_orbit.enabled = true;

        if let Some(rect) = rect {
            let selection =
                projected_selection_for_rect(viewer.as_ref(), camera, camera_transform, rect);
            let current_frame = viewer.current;
            viewer.selection.replace(current_frame, selection);
            viewer.needs_render = true;
        }
    }
}

pub fn sync_marquee_overlay(
    mut commands: Commands,
    marquee: Res<MarqueeSelectionState>,
    main_camera: Query<Entity, With<MainSceneCamera>>,
    mut overlays: Query<
        (Entity, &mut Node, &mut Visibility, Option<&UiTargetCamera>),
        With<MarqueeSelectionOverlay>,
    >,
) {
    let Ok((overlay_entity, mut node, mut visibility, target_camera)) = overlays.single_mut()
    else {
        return;
    };
    if target_camera.is_none()
        && let Ok(main_camera) = main_camera.single()
    {
        commands
            .entity(overlay_entity)
            .insert(UiTargetCamera(main_camera));
    }

    let Some(rect) = marquee.active_rect() else {
        node.display = Display::None;
        *visibility = Visibility::Hidden;
        return;
    };

    node.display = Display::Flex;
    node.left = px(rect.min.x);
    node.top = px(rect.min.y);
    node.width = px(rect.width());
    node.height = px(rect.height());
    *visibility = Visibility::Visible;
}

pub fn sync_viewer_snapshot(viewer: Res<ViewerState>, snapshot: Res<SharedViewerSnapshot>) {
    let Ok(mut snapshot) = snapshot.0.lock() else {
        return;
    };
    snapshot.current_frame = viewer.current;
    snapshot.selection = viewer.selection.clone();
}

pub fn handle_atom_clicks(
    mut clicks: MessageReader<Pointer<Click>>,
    keys: Res<ButtonInput<KeyCode>>,
    atoms: Query<&AtomIndex>,
    mut marquee: ResMut<MarqueeSelectionState>,
    mut viewer: ResMut<ViewerState>,
) {
    if marquee.take_click_suppression() {
        let _ = clicks.read().count();
        return;
    }

    let modified = keys.any_pressed([
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
    ]);

    let mut changed = false;
    for click in clicks.read() {
        if click.button != PointerButton::Primary {
            continue;
        }
        let Ok(atom_index) = atoms.get(click.entity) else {
            continue;
        };
        changed |= if modified {
            viewer.toggle_atom_selection(atom_index.0)
        } else {
            viewer.replace_atom_selection(atom_index.0)
        };
    }

    if changed {
        viewer.needs_render = true;
    }
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
    selection_highlights: Query<'w, 's, Entity, With<FrameSelectionHighlight>>,
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
        queries.selection_highlights,
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

#[cfg(test)]
mod tests {
    use super::{
        MarqueeSelectionState, marquee_drag_exceeded, selection_highlight_color,
        selection_highlight_radius, selection_mask_for_projected_positions, sync_marquee_overlay,
    };
    use crate::components::{MainSceneCamera, MarqueeSelectionOverlay};
    use bevy::ecs::system::SystemState;
    use bevy::prelude::*;

    #[test]
    fn selection_highlight_radius_keeps_a_minimum_extra_shell() {
        let small = selection_highlight_radius(0.25);
        let medium = selection_highlight_radius(0.5);

        assert!((small - 0.25) > 0.10);
        assert!((medium - 0.5) > 0.13);
    }

    #[test]
    fn selection_highlight_radius_grows_sublinearly_relative_to_atom_size() {
        let small_extra = selection_highlight_radius(0.3) - 0.3;
        let large_extra = selection_highlight_radius(1.2) - 1.2;

        assert!(large_extra > small_extra);
        assert!(selection_highlight_radius(1.2) < 1.2 * 1.2);
    }

    #[test]
    fn selection_highlight_color_is_more_pronounced() {
        let color = selection_highlight_color().to_srgba();

        assert!(color.red >= 0.99);
        assert!(color.green < 0.6);
        assert!(color.alpha >= 0.4);
    }

    #[test]
    fn marquee_threshold_requires_real_drag_distance() {
        assert!(!marquee_drag_exceeded(
            Vec2::new(10.0, 10.0),
            Vec2::new(13.0, 14.0)
        ));
        assert!(marquee_drag_exceeded(
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 10.0)
        ));
    }

    #[test]
    fn projected_selection_mask_is_depth_agnostic() {
        let rect = Rect::from_corners(Vec2::new(10.0, 10.0), Vec2::new(40.0, 40.0));
        let mask = selection_mask_for_projected_positions(
            4,
            [
                (0, Vec2::new(12.0, 12.0)),
                (1, Vec2::new(12.0, 12.0)),
                (2, Vec2::new(60.0, 60.0)),
                (3, Vec2::new(20.0, 25.0)),
            ],
            rect,
        );

        assert_eq!(mask, vec![true, true, false, true]);
    }

    #[test]
    fn sync_marquee_overlay_updates_visibility_and_bounds() {
        let mut world = World::new();
        let mut marquee = MarqueeSelectionState::default();
        marquee.begin(Vec2::new(10.0, 15.0));
        marquee.update(Vec2::new(40.0, 55.0));
        world.insert_resource(marquee);
        world.spawn((Camera::default(), MainSceneCamera));
        world.spawn((
            Node {
                display: Display::None,
                ..default()
            },
            Visibility::Hidden,
            MarqueeSelectionOverlay,
        ));

        let mut system_state: SystemState<(
            Commands,
            Res<MarqueeSelectionState>,
            Query<Entity, With<MainSceneCamera>>,
            Query<
                (Entity, &mut Node, &mut Visibility, Option<&UiTargetCamera>),
                With<MarqueeSelectionOverlay>,
            >,
        )> = SystemState::new(&mut world);

        let (commands, marquee, main_camera, overlays) = system_state.get_mut(&mut world);
        sync_marquee_overlay(commands, marquee, main_camera, overlays);
        system_state.apply(&mut world);

        let main_camera_entity = world
            .query_filtered::<Entity, With<MainSceneCamera>>()
            .single(&world)
            .expect("main camera should exist");
        let (node, visibility, target_camera) = world
            .query::<(&Node, &Visibility, &UiTargetCamera)>()
            .single(&world)
            .expect("overlay should exist");
        assert_eq!(node.display, Display::Flex);
        assert_eq!(node.left, px(10.0));
        assert_eq!(node.top, px(15.0));
        assert_eq!(node.width, px(30.0));
        assert_eq!(node.height, px(40.0));
        assert_eq!(*visibility, Visibility::Visible);
        assert_eq!(target_camera.entity(), main_camera_entity);
    }
}
