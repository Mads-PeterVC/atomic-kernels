use ak_core::PERIODIC_TABLE;
use bevy::picking::prelude::Pickable;
use bevy::prelude::*;

use crate::components::{
    InspectorHintsContainer, InspectorHintsToggle, InspectorMeasurementBody,
    InspectorMeasurementSection, InspectorPanelRoot, InspectorPanelSurface,
    InspectorSelectionBody, InspectorSelectionSection, MainSceneCamera, ToggleableUI,
};
use crate::viewer::ViewerState;

const PANEL_BACKGROUND: Color = Color::srgba(0.07, 0.08, 0.10, 0.58);
const SECTION_BACKGROUND: Color = Color::srgba(0.10, 0.11, 0.14, 0.54);
const PANEL_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);
const BODY_COLOR: Color = Color::srgb(0.76, 0.79, 0.83);
const ACCENT_COLOR: Color = Color::srgb(0.72, 0.86, 0.96);
const KEYCAP_BACKGROUND: Color = Color::srgba(0.19, 0.21, 0.25, 0.82);
const KEYCAP_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);
const KEYCAP_TEXT: Color = Color::srgb(0.89, 0.91, 0.95);
const HINT_LABEL_COLOR: Color = Color::srgb(0.70, 0.74, 0.79);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShortcutToken {
    Key(&'static str),
    Separator(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ShortcutHint {
    tokens: &'static [ShortcutToken],
    label: &'static str,
}

const CLICK_REPLACE_SELECTION_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("click")];
const SHIFT_CLICK_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("click"),
];
const SHIFT_DRAG_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("drag"),
];
const FRAME_STEP_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("A"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("D"),
];
const ZOOM_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("W"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("S"),
];
const ORBIT_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("←"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("→"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("↑"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("↓"),
];
const SNAP_VIEW_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("X"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Y"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Z"),
];
const SCREENSHOT_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("Space")];
const TOGGLE_INSPECTOR_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("U")];
const TOGGLE_HINTS_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("H")];

const SHORTCUT_HINTS: &[ShortcutHint] = &[
    ShortcutHint {
        tokens: CLICK_REPLACE_SELECTION_TOKENS,
        label: "replace selection",
    },
    ShortcutHint {
        tokens: SHIFT_CLICK_TOKENS,
        label: "toggle atom",
    },
    ShortcutHint {
        tokens: SHIFT_DRAG_TOKENS,
        label: "marquee replace",
    },
    ShortcutHint {
        tokens: FRAME_STEP_TOKENS,
        label: "step frames",
    },
    ShortcutHint {
        tokens: ZOOM_TOKENS,
        label: "zoom",
    },
    ShortcutHint {
        tokens: ORBIT_TOKENS,
        label: "orbit camera",
    },
    ShortcutHint {
        tokens: SNAP_VIEW_TOKENS,
        label: "snap to axes",
    },
    ShortcutHint {
        tokens: SCREENSHOT_TOKENS,
        label: "save screenshot",
    },
    ShortcutHint {
        tokens: TOGGLE_INSPECTOR_TOKENS,
        label: "toggle inspector",
    },
    ShortcutHint {
        tokens: TOGGLE_HINTS_TOKENS,
        label: "toggle keybindings",
    },
];

#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementStatus {
    Empty,
    NeedOneMoreAtom,
    Distance { atoms: [usize; 2], angstrom: f64 },
    Angle { atoms: [usize; 3], degrees: f64 },
    UnsupportedCount { count: usize },
}

#[derive(Resource, Clone, Debug, PartialEq, Default)]
pub struct InspectorState {
    pub selected_atoms: Vec<SelectedAtomSummary>,
    pub measurement: MeasurementStatus,
    pub hints_expanded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedAtomSummary {
    pub index: usize,
    pub symbol: String,
}

impl Default for MeasurementStatus {
    fn default() -> Self {
        Self::Empty
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf");

    commands
        .spawn_empty()
        .insert(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            padding: UiRect::all(px(18)),
            ..default()
        })
        .insert(ZIndex(20))
        .insert(Pickable::IGNORE)
        .insert(ToggleableUI)
        .insert(InspectorPanelRoot)
        .with_children(|parent| {
            parent
                .spawn_empty()
                .insert(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    width: px(280),
                    max_width: percent(28),
                    min_width: px(220),
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(16)),
                    ..default()
                })
                .insert(BackgroundColor(PANEL_BACKGROUND))
                .insert(BorderColor::all(PANEL_BORDER))
                .insert(Pickable::IGNORE)
                .insert(InspectorPanelSurface)
                .with_children(|panel| {
                    spawn_hints_section(panel, &font);
                    spawn_section(
                        panel,
                        &font,
                        "Selection",
                        "No atoms selected.\nClick an atom to inspect it.",
                        Some(InspectorSelectionSection),
                        Some(InspectorSelectionBody),
                    );
                    spawn_section(
                        panel,
                        &font,
                        "Measurement",
                        "Select 2 atoms for a distance or 3 atoms for an angle.",
                        Some(InspectorMeasurementSection),
                        Some(InspectorMeasurementBody),
                    );
                });
        });
}

fn spawn_section<S: Component, M: Component>(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    title: &str,
    body: &str,
    section_marker: Option<S>,
    marker: Option<M>,
) {
    let mut entity = parent.spawn_empty();
    entity
        .insert(Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            width: percent(100),
            padding: UiRect::all(px(10)),
            border_radius: BorderRadius::all(px(12)),
            ..default()
        })
        .insert(BackgroundColor(SECTION_BACKGROUND))
        .insert(Pickable::IGNORE)
        .with_children(|section| {
            section.spawn(section_title_bundle(title, font));
            let mut entity = section.spawn(section_body_bundle(body, font));
            if let Some(marker) = marker {
                entity.insert(marker);
            }
        });
    if let Some(section_marker) = section_marker {
        entity.insert(section_marker);
    }
}

fn section_title_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 12.0,
            ..default()
        },
        TextColor(ACCENT_COLOR),
    )
}

fn section_body_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 11.0,
            ..default()
        },
        TextColor(BODY_COLOR),
    )
}

fn spawn_hints_section(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>) {
    parent
        .spawn_empty()
        .insert(Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            width: percent(100),
            padding: UiRect::all(px(10)),
            border_radius: BorderRadius::all(px(12)),
            ..default()
        })
        .insert(BackgroundColor(SECTION_BACKGROUND))
        .insert(Pickable::IGNORE)
        .with_children(|section| {
            section
                .spawn_empty()
                .insert(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(6),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .with_children(|header| {
                    header.spawn(section_title_bundle("Keybindings", font));
                    header.spawn(hint_action_bundle("(press", font));
                    spawn_keycap(header, font, "H");
                    header.spawn((
                        Text::new("to expand)"),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(HINT_LABEL_COLOR),
                        InspectorHintsToggle,
                    ));
                });
            section
                .spawn_empty()
                .insert(Node {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .insert(InspectorHintsContainer)
                .with_children(|container| {
                    for hint in shortcut_hints() {
                        spawn_hint_row(container, font, hint);
                    }
                });
        });
}

fn spawn_hint_row(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>, hint: &ShortcutHint) {
    parent
        .spawn_empty()
        .insert(Node {
            align_items: AlignItems::Center,
            column_gap: px(8),
            ..default()
        })
        .insert(Pickable::IGNORE)
        .with_children(|row| {
            row.spawn_empty()
                .insert(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(4),
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .with_children(|tokens_parent| {
                    for token in hint.tokens {
                        match token {
                            ShortcutToken::Key(text) => spawn_keycap(tokens_parent, font, text),
                            ShortcutToken::Separator(text) => {
                                tokens_parent.spawn(hint_separator_bundle(text, font));
                            }
                        }
                    }
                });
            row.spawn(hint_action_bundle(hint.label, font));
        });
}

fn shortcut_hints() -> &'static [ShortcutHint] {
    SHORTCUT_HINTS
}

fn spawn_keycap(parent: &mut ChildSpawnerCommands<'_>, font: &Handle<Font>, text: &str) {
    parent
        .spawn_empty()
        .insert(Node {
            padding: UiRect::axes(px(7), px(3)),
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        })
        .insert(BackgroundColor(KEYCAP_BACKGROUND))
        .insert(BorderColor::all(KEYCAP_BORDER))
        .insert(Pickable::IGNORE)
        .with_children(|keycap| {
            keycap.spawn((
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size: 10.5,
                    ..default()
                },
                TextColor(KEYCAP_TEXT),
            ));
        });
}

fn hint_separator_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 10.5,
            ..default()
        },
        TextColor(HINT_LABEL_COLOR),
    )
}

fn hint_action_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 11.0,
            ..default()
        },
        TextColor(HINT_LABEL_COLOR),
    )
}

pub fn sync_inspector_state(viewer: Res<ViewerState>, mut inspector: ResMut<InspectorState>) {
    let hints_expanded = inspector.hints_expanded;
    *inspector = derive_inspector_state(viewer.as_ref());
    inspector.hints_expanded = hints_expanded;
}

pub fn sync_inspector_camera(
    mut commands: Commands,
    main_camera: Query<Entity, With<MainSceneCamera>>,
    roots: Query<(Entity, Option<&UiTargetCamera>), With<InspectorPanelRoot>>,
) {
    let Ok(main_camera) = main_camera.single() else {
        return;
    };

    for (entity, target_camera) in roots.iter() {
        if target_camera.is_none() {
            commands.entity(entity).insert(UiTargetCamera(main_camera));
        }
    }
}

pub fn sync_inspector_text(
    inspector: Res<InspectorState>,
    mut sections: ParamSet<(
        Query<&mut Node, With<InspectorSelectionSection>>,
        Query<&mut Node, With<InspectorMeasurementSection>>,
    )>,
    mut bodies: ParamSet<(
        Query<&mut Text, With<InspectorSelectionBody>>,
        Query<&mut Text, With<InspectorMeasurementBody>>,
        Query<&mut Text, With<InspectorHintsToggle>>,
    )>,
) {
    if !inspector.is_changed() {
        return;
    }

    if let Ok(mut node) = sections.p0().single_mut() {
        node.display = if inspector.selected_atoms.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
    }
    if let Ok(mut node) = sections.p1().single_mut() {
        node.display = if shows_measurement(inspector.as_ref()) {
            Display::Flex
        } else {
            Display::None
        };
    }
    if let Ok(mut text) = bodies.p0().single_mut() {
        *text = Text::new(selection_text(inspector.as_ref()));
    }
    if let Ok(mut text) = bodies.p1().single_mut() {
        *text = Text::new(measurement_text(inspector.as_ref()));
    }
    if let Ok(mut text) = bodies.p2().single_mut() {
        *text = Text::new(hints_toggle_text(inspector.as_ref()));
    }
}

pub fn derive_inspector_state(viewer: &ViewerState) -> InspectorState {
    let selected_indices = viewer.selected_atoms(viewer.current);
    let frame = viewer.traj.view(viewer.current);
    let selected_atoms = selected_indices
        .iter()
        .map(|&index| SelectedAtomSummary {
            index,
            symbol: frame
                .numbers
                .get(index)
                .map(|&number| PERIODIC_TABLE.get(number).symbol.clone())
                .unwrap_or_else(|| "?".to_string()),
        })
        .collect();

    InspectorState {
        measurement: measurement_for_selection(frame.positions, &selected_indices),
        selected_atoms,
        hints_expanded: false,
    }
}

fn measurement_for_selection(positions: &[[f64; 3]], selected_indices: &[usize]) -> MeasurementStatus {
    match selected_indices {
        [] => MeasurementStatus::Empty,
        [_] => MeasurementStatus::NeedOneMoreAtom,
        [a, b] => MeasurementStatus::Distance {
            atoms: [*a, *b],
            angstrom: distance_between(positions, *a, *b).unwrap_or(0.0),
        },
        [a, b, c] => MeasurementStatus::Angle {
            atoms: [*a, *b, *c],
            degrees: angle_between(positions, *a, *b, *c).unwrap_or(0.0),
        },
        many => MeasurementStatus::UnsupportedCount { count: many.len() },
    }
}

fn distance_between(positions: &[[f64; 3]], a: usize, b: usize) -> Option<f64> {
    let pa = positions.get(a)?;
    let pb = positions.get(b)?;
    let dx = pa[0] - pb[0];
    let dy = pa[1] - pb[1];
    let dz = pa[2] - pb[2];
    Some((dx * dx + dy * dy + dz * dz).sqrt())
}

fn angle_between(positions: &[[f64; 3]], a: usize, b: usize, c: usize) -> Option<f64> {
    let pa = positions.get(a)?;
    let pb = positions.get(b)?;
    let pc = positions.get(c)?;

    let ba = [pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]];
    let bc = [pc[0] - pb[0], pc[1] - pb[1], pc[2] - pb[2]];
    let ba_norm = vector_norm(ba);
    let bc_norm = vector_norm(bc);
    if ba_norm <= f64::EPSILON || bc_norm <= f64::EPSILON {
        return None;
    }

    let cosine = (dot(ba, bc) / (ba_norm * bc_norm)).clamp(-1.0, 1.0);
    Some(cosine.acos().to_degrees())
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vector_norm(v: [f64; 3]) -> f64 {
    dot(v, v).sqrt()
}

fn selection_text(inspector: &InspectorState) -> String {
    let count = inspector.selected_atoms.len();
    if count == 0 {
        return "Selected atoms: 0\nClick an atom to inspect it.".to_string();
    }

    let summary = if count <= 3 {
        inspector
            .selected_atoms
            .iter()
            .map(|atom| format!("#{} {}", atom.index, atom.symbol))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        inspector
            .selected_atoms
            .iter()
            .map(|atom| format!("#{}", atom.index))
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!("Selected atoms: {count}\n{summary}")
}

fn measurement_text(inspector: &InspectorState) -> String {
    match &inspector.measurement {
        MeasurementStatus::Empty => {
            "No measurement yet.\nSelect 2 atoms for a distance or 3 atoms for an angle."
                .to_string()
        }
        MeasurementStatus::NeedOneMoreAtom => {
            "Select one more atom to compute a distance.".to_string()
        }
        MeasurementStatus::Distance { atoms, angstrom } => {
            format!("Distance (#{} -> #{})\n{angstrom:.3} A", atoms[0], atoms[1])
        }
        MeasurementStatus::Angle { atoms, degrees } => {
            format!(
                "Angle (#{}-#{}-#{})\n{degrees:.2} deg",
                atoms[0], atoms[1], atoms[2]
            )
        }
        MeasurementStatus::UnsupportedCount { count } => {
            format!("Selected atoms: {count}\nMeasurements are shown for 2 or 3 atoms.")
        }
    }
}

fn hints_toggle_text(inspector: &InspectorState) -> String {
    if inspector.hints_expanded {
        "to collapse)".to_string()
    } else {
        "to expand)".to_string()
    }
}

fn shows_measurement(inspector: &InspectorState) -> bool {
    matches!(
        inspector.measurement,
        MeasurementStatus::Distance { .. } | MeasurementStatus::Angle { .. }
    )
}

pub fn toggle_hints_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector: ResMut<InspectorState>,
    mut hint_bodies: Query<&mut Node, With<InspectorHintsContainer>>,
) {
    if !keys.just_pressed(KeyCode::KeyH) {
        return;
    }

    inspector.hints_expanded = !inspector.hints_expanded;
    let display = if inspector.hints_expanded {
        Display::Block
    } else {
        Display::None
    };
    for mut node in &mut hint_bodies {
        node.display = display;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InspectorHintsContainer, InspectorHintsToggle, InspectorMeasurementBody,
        InspectorPanelRoot,
        InspectorMeasurementSection, InspectorSelectionBody, InspectorSelectionSection,
        InspectorState, MeasurementStatus, derive_inspector_state, shortcut_hints,
        sync_inspector_camera, sync_inspector_text, toggle_hints_visibility,
    };
    use crate::components::MainSceneCamera;
    use crate::viewer::ViewerState;
    use ak_core::{Structure, Trajectory};
    use bevy::prelude::*;

    fn structure(points: &[[f64; 3]], numbers: &[i32]) -> Structure {
        Structure::new(
            points.to_vec(),
            numbers.to_vec(),
            [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
            [false, false, false],
        )
    }

    #[test]
    fn derive_inspector_state_reports_distance_for_two_selected_atoms() {
        let traj = Trajectory::new(vec![structure(&[[0.0, 0.0, 0.0], [0.0, 3.0, 4.0]], &[1, 8])]);
        let mut viewer = ViewerState::new(traj, 0);
        viewer.selection.replace(0, vec![true, true]);

        let inspector = derive_inspector_state(&viewer);

        assert_eq!(inspector.selected_atoms.len(), 2);
        assert_eq!(inspector.selected_atoms[0].symbol, "H");
        assert!(matches!(
            inspector.measurement,
            MeasurementStatus::Distance { angstrom, .. } if (angstrom - 5.0).abs() < 1e-9
        ));
    }

    #[test]
    fn derive_inspector_state_reports_angle_for_three_selected_atoms() {
        let traj = Trajectory::new(vec![structure(
            &[[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[1, 6, 8],
        )]);
        let mut viewer = ViewerState::new(traj, 0);
        viewer.selection.replace(0, vec![true, true, true]);

        let inspector = derive_inspector_state(&viewer);

        assert!(matches!(
            inspector.measurement,
            MeasurementStatus::Angle { degrees, .. } if (degrees - 90.0).abs() < 1e-9
        ));
    }

    #[test]
    fn derive_inspector_state_reports_selection_limits() {
        let traj = Trajectory::new(vec![structure(
            &[
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [3.0, 0.0, 0.0],
            ],
            &[1, 1, 1, 1],
        )]);
        let mut viewer = ViewerState::new(traj, 0);

        let empty = derive_inspector_state(&viewer);
        assert_eq!(empty.measurement, MeasurementStatus::Empty);

        viewer.selection.replace(0, vec![true, false, false, false]);
        let one = derive_inspector_state(&viewer);
        assert_eq!(one.measurement, MeasurementStatus::NeedOneMoreAtom);

        viewer.selection.replace(0, vec![true, true, true, true]);
        let many = derive_inspector_state(&viewer);
        assert_eq!(
            many.measurement,
            MeasurementStatus::UnsupportedCount { count: 4 }
        );
    }

    #[test]
    fn derive_inspector_state_uses_current_frame_coordinates() {
        let traj = Trajectory::new(vec![
            structure(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], &[1, 1]),
            structure(&[[0.0, 0.0, 0.0], [2.5, 0.0, 0.0]], &[1, 1]),
        ]);
        let mut viewer = ViewerState::new(traj, 0);
        viewer.selection.replace(0, vec![true, true]);
        viewer.selection.replace(1, vec![true, true]);
        viewer.current = 1;

        let inspector = derive_inspector_state(&viewer);

        assert!(matches!(
            inspector.measurement,
            MeasurementStatus::Distance { angstrom, .. } if (angstrom - 2.5).abs() < 1e-9
        ));
    }

    #[test]
    fn shortcut_hints_cover_current_viewer_controls() {
        let labels: Vec<&str> = shortcut_hints().iter().map(|hint| hint.label).collect();

        assert!(labels.contains(&"replace selection"));
        assert!(labels.contains(&"toggle atom"));
        assert!(labels.contains(&"marquee replace"));
        assert!(labels.contains(&"step frames"));
        assert!(labels.contains(&"zoom"));
        assert!(labels.contains(&"orbit camera"));
        assert!(labels.contains(&"snap to axes"));
        assert!(labels.contains(&"save screenshot"));
        assert!(labels.contains(&"toggle inspector"));
        assert!(labels.contains(&"toggle keybindings"));
    }

    #[test]
    fn sync_inspector_text_updates_all_sections() {
        let mut app = App::new();
        app.insert_resource(InspectorState {
            selected_atoms: vec![
                super::SelectedAtomSummary {
                    index: 0,
                    symbol: "H".to_string(),
                },
                super::SelectedAtomSummary {
                    index: 2,
                    symbol: "O".to_string(),
                },
            ],
            measurement: MeasurementStatus::Distance {
                atoms: [0, 2],
                angstrom: 1.2345,
            },
            hints_expanded: true,
        });
        let selection = app
            .world_mut()
            .spawn((
                Node {
                    display: Display::Flex,
                    ..default()
                },
                InspectorSelectionSection,
            ))
            .with_child((Text::new(""), InspectorSelectionBody))
            .id();
        let measurement = app
            .world_mut()
            .spawn((
                Node {
                    display: Display::Flex,
                    ..default()
                },
                InspectorMeasurementSection,
            ))
            .with_child((Text::new(""), InspectorMeasurementBody))
            .id();
        let hints_toggle = app
            .world_mut()
            .spawn((Text::new(""), InspectorHintsToggle))
            .id();
        app.add_systems(Update, sync_inspector_text);

        app.update();

        assert!(
            app.world()
                .entity(selection)
                .get::<Children>()
                .and_then(|children| children.first().copied())
                .and_then(|child| app.world().get::<Text>(child))
                .unwrap()
                .0
                .contains("#0 H, #2 O")
        );
        assert!(
            app.world()
                .entity(measurement)
                .get::<Children>()
                .and_then(|children| children.first().copied())
                .and_then(|child| app.world().get::<Text>(child))
                .unwrap()
                .0
                .contains("1.234")
        );
        assert_eq!(
            app.world().get::<Text>(hints_toggle).unwrap().0,
            "to collapse)"
        );
    }

    #[test]
    fn sync_inspector_text_hides_selection_section_when_empty() {
        let mut app = App::new();
        app.insert_resource(InspectorState::default());
        let section = app
            .world_mut()
            .spawn((
                Node {
                    display: Display::Flex,
                    ..default()
                },
                InspectorSelectionSection,
            ))
            .id();
        app.world_mut().entity_mut(section).with_child((Text::new(""), InspectorSelectionBody));
        app.world_mut()
            .spawn((
                Node {
                    display: Display::Flex,
                    ..default()
                },
                InspectorMeasurementSection,
            ))
            .with_child((Text::new(""), InspectorMeasurementBody));
        app.world_mut().spawn((Text::new(""), InspectorHintsToggle));
        app.add_systems(Update, sync_inspector_text);

        app.update();

        assert_eq!(app.world().get::<Node>(section).unwrap().display, Display::None);
    }

    #[test]
    fn sync_inspector_text_hides_measurement_section_without_result() {
        let mut app = App::new();
        app.insert_resource(InspectorState::default());
        app.world_mut().spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorSelectionSection,
        ));
        let section = app
            .world_mut()
            .spawn((
                Node {
                    display: Display::Flex,
                    ..default()
                },
                InspectorMeasurementSection,
            ))
            .id();
        app.world_mut()
            .entity_mut(section)
            .with_child((Text::new(""), InspectorMeasurementBody));
        app.world_mut().spawn((Text::new(""), InspectorHintsToggle));
        app.add_systems(Update, sync_inspector_text);

        app.update();

        assert_eq!(app.world().get::<Node>(section).unwrap().display, Display::None);
    }

    #[test]
    fn sync_inspector_camera_targets_main_scene_camera() {
        let mut app = App::new();
        let main_camera = app.world_mut().spawn((Camera::default(), MainSceneCamera)).id();
        let inspector = app.world_mut().spawn(InspectorPanelRoot).id();
        app.add_systems(Update, sync_inspector_camera);

        app.update();

        assert_eq!(
            app.world().get::<UiTargetCamera>(inspector).unwrap().0,
            main_camera
        );
    }

    #[test]
    fn toggle_hints_visibility_flips_body_and_state_on_h_press() {
        let mut app = App::new();
        app.insert_resource(InspectorState::default());
        let entity = app
            .world_mut()
            .spawn((
                InspectorHintsContainer,
                Node {
                    display: Display::None,
                    ..default()
                },
            ))
            .id();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyH);
        app.insert_resource(keys);
        app.add_systems(Update, toggle_hints_visibility);

        app.update();

        assert!(app.world().resource::<InspectorState>().hints_expanded);
        assert_eq!(
            app.world().get::<Node>(entity).unwrap().display,
            Display::Block
        );
    }
}
