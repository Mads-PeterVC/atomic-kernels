use crate::components::{FrameMeasurementCue, FrameSelectionHighlight};
use crate::structure_position_to_world;
use crate::viewer::{DisplayAtom, SelectedImageAtom};
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::picking::prelude::Pickable;
use bevy::prelude::*;

pub(super) const ANGLE_CUE_RADIUS_FACTOR: f32 = 0.35;
pub(super) const ANGLE_CUE_RADIUS_MIN: f32 = 0.20;
pub(super) const ANGLE_CUE_RADIUS_MAX: f32 = 0.75;
pub(super) const ANGLE_CUE_RAY_LENGTH_FACTOR: f32 = 1.32;
pub(super) const ANGLE_CUE_RAY_START_FACTOR: f32 = 0.88;
pub(super) const ANGLE_CUE_RAY_RADIUS: f32 = 0.05;
pub(super) const ANGLE_CUE_ARC_RADIUS: f32 = 0.042;
pub(super) const ANGLE_CUE_ARC_SEGMENTS: usize = 12;
pub(super) const ANGLE_CUE_MIN_ANGLE_RAD: f32 = 2.0_f32.to_radians();
pub(super) const ANGLE_CUE_COLOR: Color = Color::srgba(1.0, 0.84, 0.22, 0.98);
pub(super) const ANGLE_VERTEX_HIGHLIGHT_COLOR: Color = Color::srgba(1.0, 0.86, 0.24, 0.56);

#[derive(Clone, Debug, PartialEq)]
pub(super) struct AngleMeasurementCue {
    pub(super) atoms: [SelectedImageAtom; 3],
    pub(super) vertex: Vec3,
    pub(super) first_direction: Vec3,
    pub(super) second_direction: Vec3,
    pub(super) radius: f32,
    angle_radians: f32,
}

pub(super) fn angle_measurement_cue_from_positions(
    positions: &[DisplayAtom],
    selected_indices: &[SelectedImageAtom],
    vertex_atom_radius: f32,
) -> Option<AngleMeasurementCue> {
    let &[a, b, c] = selected_indices else {
        return None;
    };

    let pa =
        structure_position_to_world(positions.iter().find(|atom| atom.identity == a)?.position);
    let pb =
        structure_position_to_world(positions.iter().find(|atom| atom.identity == b)?.position);
    let pc =
        structure_position_to_world(positions.iter().find(|atom| atom.identity == c)?.position);

    let first = pa - pb;
    let second = pc - pb;
    let first_length = first.length();
    let second_length = second.length();
    if first_length <= f32::EPSILON || second_length <= f32::EPSILON {
        return None;
    }

    let first_direction = first / first_length;
    let second_direction = second / second_length;
    let angle_radians = first_direction
        .dot(second_direction)
        .clamp(-1.0, 1.0)
        .acos();
    if !angle_radians.is_finite() || angle_radians < ANGLE_CUE_MIN_ANGLE_RAD {
        return None;
    }

    let geometry_radius = (first_length.min(second_length) * ANGLE_CUE_RADIUS_FACTOR)
        .clamp(ANGLE_CUE_RADIUS_MIN, ANGLE_CUE_RADIUS_MAX);
    let minimum_visible_radius = selection_highlight_radius(vertex_atom_radius, true) + 0.14;
    let radius = geometry_radius.max(minimum_visible_radius);

    Some(AngleMeasurementCue {
        atoms: [a, b, c],
        vertex: pb,
        first_direction,
        second_direction,
        radius,
        angle_radians,
    })
}

pub(super) fn selection_highlight_radius(atom_radius: f32, is_vertex: bool) -> f32 {
    let base = atom_radius + 0.08 + atom_radius * 0.12;
    if is_vertex {
        base + 0.06 + atom_radius * 0.06
    } else {
        base
    }
}

pub(super) fn selection_highlight_color(is_vertex: bool) -> Color {
    if is_vertex {
        ANGLE_VERTEX_HIGHLIGHT_COLOR
    } else {
        Color::srgba(1.0, 0.55, 0.08, 0.42)
    }
}

pub(super) fn render_selection_highlights(
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

fn arc_points(cue: &AngleMeasurementCue) -> Option<Vec<Vec3>> {
    let normal = cue
        .first_direction
        .cross(cue.second_direction)
        .normalize_or_zero();
    if normal.length_squared() <= f32::EPSILON {
        return None;
    }

    let steps = ANGLE_CUE_ARC_SEGMENTS.max(2);
    Some(
        (0..=steps)
            .map(|step| {
                let fraction = step as f32 / steps as f32;
                let rotation = Quat::from_axis_angle(normal, cue.angle_radians * fraction);
                cue.vertex + rotation * cue.first_direction * cue.radius
            })
            .collect(),
    )
}

fn cylinder_transform_between(p0: Vec3, p1: Vec3, radius: f32) -> Option<Transform> {
    let delta = p1 - p0;
    let length = delta.length();
    if length <= f32::EPSILON {
        return None;
    }

    let midpoint = (p0 + p1) * 0.5;
    let direction = delta / length;
    Some(Transform {
        translation: midpoint,
        rotation: Quat::from_rotation_arc(Vec3::Y, direction),
        scale: Vec3::new(radius, length * 0.5, radius),
    })
}

pub(super) fn render_angle_measurement_cue(
    cue: &AngleMeasurementCue,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: ANGLE_CUE_COLOR,
        emissive: LinearRgba::from(ANGLE_CUE_COLOR) * 0.15,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    let cylinder_mesh = meshes.add(Cylinder::new(1.0, 2.0));

    let ray_length = cue.radius * ANGLE_CUE_RAY_LENGTH_FACTOR;
    for direction in [cue.first_direction, cue.second_direction] {
        let start = cue.vertex + direction * (cue.radius * ANGLE_CUE_RAY_START_FACTOR);
        let Some(transform) = cylinder_transform_between(
            start,
            cue.vertex + direction * ray_length,
            ANGLE_CUE_RAY_RADIUS,
        ) else {
            continue;
        };
        commands.spawn((
            Mesh3d(cylinder_mesh.clone()),
            MeshMaterial3d(material.clone()),
            transform,
            FrameMeasurementCue,
            Pickable::IGNORE,
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }

    let Some(points) = arc_points(cue) else {
        return;
    };
    for segment in points.windows(2) {
        let Some(transform) =
            cylinder_transform_between(segment[0], segment[1], ANGLE_CUE_ARC_RADIUS)
        else {
            continue;
        };
        commands.spawn((
            Mesh3d(cylinder_mesh.clone()),
            MeshMaterial3d(material.clone()),
            transform,
            FrameMeasurementCue,
            Pickable::IGNORE,
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }
}
