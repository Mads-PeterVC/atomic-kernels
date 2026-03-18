use ak_core::PERIODIC_TABLE;
use bevy::prelude::*;

use crate::components::{
    InspectorHintsContainer, InspectorHintsToggle, InspectorMeasurementBody,
    InspectorMeasurementSection, InspectorPanelRoot, InspectorSelectionBody,
    InspectorSelectionSection, MainSceneCamera,
};
use crate::viewer::ViewerState;

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
    pub image_offset: [i32; 3],
}

impl Default for MeasurementStatus {
    fn default() -> Self {
        Self::Empty
    }
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
    let selected_indices = viewer.selected_images(viewer.current);
    let frame = viewer.traj.view(viewer.current);
    let selected_atoms = selected_indices
        .iter()
        .map(|selected| SelectedAtomSummary {
            index: selected.atom_index,
            symbol: frame
                .numbers
                .get(selected.atom_index)
                .map(|&number| PERIODIC_TABLE.get(number).symbol.clone())
                .unwrap_or_else(|| "?".to_string()),
            image_offset: selected.image_offset,
        })
        .collect();

    InspectorState {
        measurement: measurement_for_selection(&viewer.display_atoms(viewer.current), &selected_indices),
        selected_atoms,
        hints_expanded: false,
    }
}

fn measurement_for_selection(
    positions: &[crate::viewer::DisplayAtom],
    selected_indices: &[crate::viewer::SelectedImageAtom],
) -> MeasurementStatus {
    match selected_indices {
        [] => MeasurementStatus::Empty,
        [_] => MeasurementStatus::NeedOneMoreAtom,
        [a, b] => MeasurementStatus::Distance {
            atoms: [a.atom_index, b.atom_index],
            angstrom: distance_between(positions, *a, *b).unwrap_or(0.0),
        },
        [a, b, c] => MeasurementStatus::Angle {
            atoms: [a.atom_index, b.atom_index, c.atom_index],
            degrees: angle_between(positions, *a, *b, *c).unwrap_or(0.0),
        },
        many => MeasurementStatus::UnsupportedCount { count: many.len() },
    }
}

fn distance_between(
    positions: &[crate::viewer::DisplayAtom],
    a: crate::viewer::SelectedImageAtom,
    b: crate::viewer::SelectedImageAtom,
) -> Option<f64> {
    let pa = positions.iter().find(|atom| atom.identity == a)?.position;
    let pb = positions.iter().find(|atom| atom.identity == b)?.position;
    let dx = pa[0] - pb[0];
    let dy = pa[1] - pb[1];
    let dz = pa[2] - pb[2];
    Some((dx * dx + dy * dy + dz * dz).sqrt())
}

fn angle_between(
    positions: &[crate::viewer::DisplayAtom],
    a: crate::viewer::SelectedImageAtom,
    b: crate::viewer::SelectedImageAtom,
    c: crate::viewer::SelectedImageAtom,
) -> Option<f64> {
    let pa = positions.iter().find(|atom| atom.identity == a)?.position;
    let pb = positions.iter().find(|atom| atom.identity == b)?.position;
    let pc = positions.iter().find(|atom| atom.identity == c)?.position;

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
            .map(|atom| {
                if atom.image_offset == [0, 0, 0] {
                    format!("#{} {}", atom.index, atom.symbol)
                } else {
                    format!(
                        "#{} {} [{}, {}, {}]",
                        atom.index,
                        atom.symbol,
                        atom.image_offset[0],
                        atom.image_offset[1],
                        atom.image_offset[2]
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        inspector
            .selected_atoms
            .iter()
            .map(|atom| {
                if atom.image_offset == [0, 0, 0] {
                    format!("#{}", atom.index)
                } else {
                    format!(
                        "#{} [{}, {}, {}]",
                        atom.index, atom.image_offset[0], atom.image_offset[1], atom.image_offset[2]
                    )
                }
            })
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
    let _ = inspector;
    ")".to_string()
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
        Display::Flex
    } else {
        Display::None
    };
    for mut node in &mut hint_bodies {
        node.display = display;
    }
}
