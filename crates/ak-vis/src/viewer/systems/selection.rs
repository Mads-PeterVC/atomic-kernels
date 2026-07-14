use crate::components::{DisplayAtomIdentity, MainSceneCamera, MarqueeSelectionOverlay};
use crate::structure_position_to_world;
use crate::viewer::{SelectedImageAtom, SelectionFrames, ViewerState};
use bevy::picking::prelude::{Click, Pointer, PointerButton};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

const MARQUEE_DRAG_THRESHOLD: f32 = 6.0;

#[derive(Resource, Debug, Default, Clone)]
pub struct MarqueeSelectionState {
    drag_start: Option<Vec2>,
    drag_current: Option<Vec2>,
    drag_active: bool,
    suppress_click_once: bool,
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

pub(super) fn marquee_drag_exceeded(start: Vec2, current: Vec2) -> bool {
    start.distance(current) >= MARQUEE_DRAG_THRESHOLD
}

fn marquee_rect(start: Vec2, current: Vec2) -> Rect {
    Rect::from_corners(start, current)
}

pub(super) fn image_selection_for_projected_positions(
    projected_positions: impl IntoIterator<Item = (SelectedImageAtom, Vec2)>,
    rect: Rect,
) -> Vec<SelectedImageAtom> {
    let mut selection = Vec::new();
    for (index, viewport_position) in projected_positions {
        if rect.contains(viewport_position) && !selection.contains(&index) {
            selection.push(index);
        }
    }
    selection
}

fn projected_selection_for_rect(
    viewer: &ViewerState,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    rect: Rect,
) -> Vec<SelectedImageAtom> {
    let projected = viewer
        .display_atoms(viewer.current)
        .into_iter()
        .filter_map(|atom| {
            camera
                .world_to_viewport(camera_transform, structure_position_to_world(atom.position))
                .ok()
                .map(|viewport_position| (atom.identity, viewport_position))
        });
    image_selection_for_projected_positions(projected, rect)
}

impl MarqueeSelectionState {
    pub(crate) fn is_tracking(&self) -> bool {
        self.drag_start.is_some()
    }

    pub(crate) fn begin(&mut self, cursor: Vec2) {
        self.drag_start = Some(cursor);
        self.drag_current = Some(cursor);
        self.drag_active = false;
    }

    pub(super) fn update(&mut self, cursor: Vec2) {
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
            viewer.image_selection.replace(current_frame, selection);
            let selected_images = viewer.image_selection.selected(current_frame);
            let atom_count = viewer.traj.view(current_frame).positions.len();
            viewer.selection.replace(
                current_frame,
                SelectionFrames::mask_from_images(atom_count, &selected_images),
            );
            viewer.selection.set_order(
                current_frame,
                SelectionFrames::ordered_atoms_from_images(&selected_images),
            );
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

pub fn handle_atom_clicks(
    mut clicks: MessageReader<Pointer<Click>>,
    keys: Res<ButtonInput<KeyCode>>,
    atoms: Query<&DisplayAtomIdentity>,
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
        let selected = SelectedImageAtom {
            atom_index: atom_index.atom_index,
            image_offset: atom_index.image_offset,
        };
        changed |= if modified {
            viewer.toggle_atom_selection(selected)
        } else {
            viewer.replace_atom_selection(selected)
        };
    }

    if changed {
        viewer.needs_render = true;
    }
}
