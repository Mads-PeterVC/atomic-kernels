use crate::components::{
    FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameMeasurementCue,
    FrameSelectionHighlight,
};
use crate::render::{render_atoms, render_axis, render_bonds, render_cell, render_faces};
use crate::viewer::controls::{default_camera_state, despawn_current_frame};
use crate::viewer::{
    AppearanceChannel, AtomAppearanceRule, BallAndStickStyle, BondList, BondScope, CameraState,
    RenderStyle, SelectedImageAtom, ViewerConfig, ViewerState,
};
use crate::visuals::atom_visual::AtomIdentity;
use crate::visuals::{BondVisual, FaceVisual};
use crate::{ColorScheme, convert_axis, convert_cell, named_palette, structure_position_to_world};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::collections::HashMap;

use super::measurement::{
    angle_measurement_cue_from_positions, render_angle_measurement_cue,
    render_selection_highlights, selection_highlight_color, selection_highlight_radius,
};

fn normalized_scalar_values_for_rule(
    viewer: &ViewerState,
    rule: &AtomAppearanceRule,
) -> Option<Vec<Option<f32>>> {
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
                    Some(((value - min) / span).clamp(0.0, 1.0))
                } else {
                    None
                }
            })
            .collect(),
    )
}

fn scalar_colors_for_current_frame(viewer: &ViewerState) -> Option<Vec<Option<Color>>> {
    if viewer.atom_appearance_rules.is_empty() {
        return None;
    }

    let atom_count = viewer.traj.view(viewer.current).positions.len();
    let mut layered_colors = vec![None; atom_count];
    let mut applied_any = false;

    for rule in viewer
        .atom_appearance_rules
        .iter()
        .filter(|rule| rule.channel == AppearanceChannel::Color)
    {
        let Some(rule_values) = normalized_scalar_values_for_rule(viewer, rule) else {
            continue;
        };
        let Some(palette) = rule.palette.as_ref() else {
            continue;
        };
        for (slot, value) in layered_colors.iter_mut().zip(rule_values) {
            if let Some(value) = value {
                *slot = Some(palette.color(value));
                applied_any = true;
            }
        }
    }

    applied_any.then_some(layered_colors)
}

fn scalar_material_channel_for_current_frame(
    viewer: &ViewerState,
    channel: AppearanceChannel,
) -> Option<Vec<Option<f32>>> {
    if viewer.atom_appearance_rules.is_empty() {
        return None;
    }

    let atom_count = viewer.traj.view(viewer.current).positions.len();
    let mut layered_values = vec![None; atom_count];
    let mut applied_any = false;

    for rule in viewer
        .atom_appearance_rules
        .iter()
        .filter(|rule| rule.channel == channel)
    {
        let Some(rule_values) = normalized_scalar_values_for_rule(viewer, rule) else {
            continue;
        };
        for (slot, value) in layered_values.iter_mut().zip(rule_values) {
            if value.is_some() {
                *slot = value;
                applied_any = true;
            }
        }
    }

    applied_any.then_some(layered_values)
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

fn displayed_atom_visuals_for_current_frame(
    viewer: &ViewerState,
    atom_palette_name: &str,
) -> Vec<crate::AtomVisual> {
    let view = viewer.traj.view(viewer.current);
    let atom_palette = named_palette(atom_palette_name);
    let scalar_colors = scalar_colors_for_current_frame(viewer);
    let scalar_metallic =
        scalar_material_channel_for_current_frame(viewer, AppearanceChannel::Metallic);
    let scalar_roughness =
        scalar_material_channel_for_current_frame(viewer, AppearanceChannel::PerceptualRoughness);
    let ball_and_stick_styles = resolved_atom_styles_for_current_frame(viewer);

    viewer
        .display_atoms(viewer.current)
        .into_iter()
        .map(|atom| {
            let atomic_number = view.numbers[atom.identity.atom_index];
            let mut color = atom_palette.color(&view, atom.identity.atom_index);
            let mut material = atom_palette.material(atomic_number);
            if let Some(colors) = scalar_colors.as_ref()
                && let Some(mapped) = colors[atom.identity.atom_index]
            {
                color = mapped;
            }
            if let Some(values) = scalar_metallic.as_ref()
                && let Some(mapped) = values[atom.identity.atom_index]
            {
                material.metallic = mapped;
            }
            if let Some(values) = scalar_roughness.as_ref()
                && let Some(mapped) = values[atom.identity.atom_index]
            {
                material.perceptual_roughness = mapped;
            }

            let mut radius = 0.9
                * ak_core::PERIODIC_TABLE
                    .get(view.numbers[atom.identity.atom_index])
                    .covalent_radius as f32;
            if let Some(styles) = ball_and_stick_styles.as_ref()
                && let Some(style) = styles[atom.identity.atom_index]
            {
                radius *= style.atom_scale;
            }

            if !atom.is_main_cell && viewer.supercell.ghost_repeated_images {
                let srgb = color.to_srgba();
                color = Color::srgba(
                    (srgb.red * 0.55) + 0.35,
                    (srgb.green * 0.55) + 0.35,
                    (srgb.blue * 0.55) + 0.35,
                    (srgb.alpha * 0.35).max(0.18),
                );
            }

            crate::AtomVisual {
                atom_identity: AtomIdentity {
                    atom_index: atom.identity.atom_index,
                    image_offset: atom.identity.image_offset,
                },
                position: structure_position_to_world(atom.position).to_array(),
                color,
                material,
                radius,
            }
        })
        .collect()
}

fn selection_highlight_visuals_for_current_frame(
    viewer: &ViewerState,
    atom_palette_name: &str,
) -> Vec<crate::AtomVisual> {
    let selection = viewer.selected_images(viewer.current);
    if selection.is_empty() {
        return Vec::new();
    }

    let emphasized_vertex = angle_measurement_cue_for_current_frame(viewer).map(|cue| cue.atoms[1]);
    displayed_atom_visuals_for_current_frame(viewer, atom_palette_name)
        .into_iter()
        .filter_map(|mut visual| {
            let selected = SelectedImageAtom {
                atom_index: visual.atom_index(),
                image_offset: visual.atom_identity.image_offset,
            };
            if !selection.contains(&selected) {
                return None;
            }
            let is_vertex = emphasized_vertex == Some(selected);
            visual.radius = selection_highlight_radius(visual.radius, is_vertex);
            visual.color = selection_highlight_color(is_vertex);
            Some(visual)
        })
        .collect()
}

fn angle_measurement_cue_for_current_frame(
    viewer: &ViewerState,
) -> Option<super::measurement::AngleMeasurementCue> {
    let selected = viewer.selected_images(viewer.current);
    let displayed_atoms = viewer.display_atoms(viewer.current);
    let vertex_atom_radius = displayed_atom_visuals_for_current_frame(viewer, "jmol")
        .into_iter()
        .find(|visual| {
            selected.get(1).copied()
                == Some(SelectedImageAtom {
                    atom_index: visual.atom_index(),
                    image_offset: visual.atom_identity.image_offset,
                })
        })
        .map(|visual| visual.radius)
        .unwrap_or(0.0);
    angle_measurement_cue_from_positions(&displayed_atoms, &selected, vertex_atom_radius)
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

    let atom_visuals =
        displayed_atom_visuals_for_current_frame(viewer, &config.render.atom_palette);
    render_atoms(
        atom_visuals,
        commands,
        materials,
        meshes,
        config.render.ico_subdiv,
    );

    let selection_highlights =
        selection_highlight_visuals_for_current_frame(viewer, &config.render.atom_palette);
    if !selection_highlights.is_empty() {
        render_selection_highlights(
            selection_highlights,
            commands,
            materials,
            meshes,
            config.render.ico_subdiv,
        );
    }

    if let Some(cue) = angle_measurement_cue_for_current_frame(viewer) {
        render_angle_measurement_cue(&cue, commands, materials, meshes);
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
        let cell_visuals = convert_cell(&viewer.traj.view(viewer.current), config.color.cell_color);
        render_cell(cell_visuals, commands, materials, meshes);
    }

    if config.render.show_axes {
        let axis_visuals = convert_axis(&viewer.traj.view(viewer.current));
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

#[derive(SystemParam)]
pub struct RenderFrameQueries<'w, 's> {
    atoms: Query<'w, 's, Entity, With<FrameAtom>>,
    selection_highlights: Query<'w, 's, Entity, With<FrameSelectionHighlight>>,
    measurement_cues: Query<'w, 's, Entity, With<FrameMeasurementCue>>,
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
        queries.measurement_cues,
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
