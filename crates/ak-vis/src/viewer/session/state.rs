use ak_core::Trajectory;
use bevy::prelude::{Resource, Vec3};
use std::collections::HashMap;

use super::snapshot::{clamp_frame, scaled_cell_translation, supercell_offsets};
use super::{
    AtomAppearanceRule, BondFrames, DisplayAtom, FaceFrames, ImageSelectionFrames, RenderStyleRule,
    SelectedImageAtom, SelectionFrames, SupercellSettings,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitMotion {
    pub yaw_rate: f32,
    pub pitch_rate: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraView {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct CameraState {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub needs_apply: bool,
    pub motion: Option<OrbitMotion>,
}

#[derive(Resource)]
pub struct ViewerState {
    pub traj: Trajectory,
    pub current: usize,
    pub follow_tail: bool,
    pub atom_scalars: HashMap<String, Vec<Option<Vec<f32>>>>,
    pub atom_appearance_rules: Vec<AtomAppearanceRule>,
    pub bonds: BondFrames,
    pub faces: FaceFrames,
    pub render_style_rules: Vec<RenderStyleRule>,
    pub selection: SelectionFrames,
    pub image_selection: ImageSelectionFrames,
    pub supercell: SupercellSettings,
    pub needs_render: bool,
    pub needs_camera_reset: bool,
}

impl ViewerState {
    pub fn new(traj: Trajectory, initial_frame: usize) -> Self {
        let frame_count = traj.len();
        let current = clamp_frame(initial_frame, traj.len());
        let selection = SelectionFrames::new(&traj);
        let image_selection = ImageSelectionFrames::new(frame_count);
        Self {
            traj,
            current,
            follow_tail: false,
            atom_scalars: HashMap::new(),
            atom_appearance_rules: Vec::new(),
            bonds: BondFrames::new(frame_count),
            faces: FaceFrames::new(frame_count),
            render_style_rules: Vec::new(),
            selection,
            image_selection,
            supercell: SupercellSettings::default(),
            needs_render: true,
            needs_camera_reset: true,
        }
    }

    pub fn trajectory_len(&self) -> usize {
        self.traj.len()
    }

    pub fn has_frames(&self) -> bool {
        !self.traj.is_empty()
    }

    pub fn step_frame(&mut self, delta: isize) -> bool {
        if self.traj.is_empty() {
            return false;
        }

        let next = self.current as isize + delta;
        if next < 0 || next >= self.traj.len() as isize {
            return false;
        }

        let next = next as usize;
        if next == self.current {
            return false;
        }

        self.set_current_frame_index(next)
    }

    pub fn set_current_frame_index(&mut self, index: usize) -> bool {
        if index >= self.traj.len() || index == self.current {
            return false;
        }

        let previous = self.current;
        self.copy_selection_between_compatible_frames(previous, index);
        self.current = index;
        self.needs_render = true;
        self.needs_camera_reset = self.cell_changed(previous, self.current);
        true
    }

    pub fn is_at_last_frame(&self) -> bool {
        self.has_frames() && self.current + 1 >= self.traj.len()
    }

    pub fn selected_atoms(&self, frame_index: usize) -> Vec<usize> {
        self.selection.selected_indices(frame_index)
    }

    pub fn selected_images(&self, frame_index: usize) -> Vec<SelectedImageAtom> {
        let selected = self.image_selection.selected(frame_index);
        if selected.is_empty() {
            self.selection.selected_main_images(frame_index)
        } else {
            selected
        }
    }

    pub fn current_selection(&self) -> &[bool] {
        self.selection.get(self.current).unwrap_or(&[])
    }

    pub fn current_image_selection(&self) -> &[SelectedImageAtom] {
        self.image_selection.get(self.current).unwrap_or(&[])
    }

    pub fn current_supercell(&self) -> SupercellSettings {
        self.supercell
    }

    pub fn replace_atom_selection(&mut self, atom: SelectedImageAtom) -> bool {
        let changed = self.image_selection.replace_single(self.current, atom);
        if changed {
            self.sync_main_selection_from_images(self.current);
        }
        changed
    }

    pub fn toggle_atom_selection(&mut self, atom: SelectedImageAtom) -> bool {
        let changed = self.image_selection.toggle(self.current, atom);
        if changed {
            self.sync_main_selection_from_images(self.current);
        }
        changed
    }

    pub fn display_atoms(&self, frame_index: usize) -> Vec<DisplayAtom> {
        if frame_index >= self.traj.len() {
            return Vec::new();
        }

        let view = self.traj.view(frame_index);
        let offsets = supercell_offsets(self.supercell.repeats);
        let mut atoms = Vec::with_capacity(view.positions.len() * offsets.len());
        for image_offset in offsets {
            let shift = scaled_cell_translation(view.cell, image_offset);
            for (atom_index, position) in view.positions.iter().copied().enumerate() {
                atoms.push(DisplayAtom {
                    identity: SelectedImageAtom {
                        atom_index,
                        image_offset,
                    },
                    position: [
                        position[0] + shift[0],
                        position[1] + shift[1],
                        position[2] + shift[2],
                    ],
                    is_main_cell: image_offset == [0, 0, 0],
                });
            }
        }
        atoms
    }

    pub(super) fn cell_changed(&self, old_index: usize, new_index: usize) -> bool {
        self.traj.view(old_index).cell != self.traj.view(new_index).cell
    }

    fn frames_have_matching_atom_identity(&self, source_index: usize, target_index: usize) -> bool {
        if source_index >= self.traj.len() || target_index >= self.traj.len() {
            return false;
        }

        let source = self.traj.view(source_index);
        let target = self.traj.view(target_index);
        source.positions.len() == target.positions.len() && source.numbers == target.numbers
    }

    pub(super) fn copy_selection_between_compatible_frames(
        &mut self,
        source_index: usize,
        target_index: usize,
    ) {
        if !self.frames_have_matching_atom_identity(source_index, target_index) {
            return;
        }

        let selection = self.selected_images(source_index);
        if !self.validate_image_selection(target_index, &selection) {
            return;
        }

        self.image_selection.replace(target_index, selection);
        self.sync_main_selection_from_images(target_index);
    }

    pub(super) fn validate_scalar_values(&self, frame_index: usize, values: &[f32]) -> bool {
        frame_index < self.traj.len() && self.traj.view(frame_index).positions.len() == values.len()
    }

    pub(super) fn validate_selection(&self, frame_index: usize, selection: &[bool]) -> bool {
        frame_index < self.traj.len()
            && self.traj.view(frame_index).positions.len() == selection.len()
    }

    pub(super) fn validate_image_selection(
        &self,
        frame_index: usize,
        selection: &[SelectedImageAtom],
    ) -> bool {
        if frame_index >= self.traj.len() {
            return false;
        }
        let atom_count = self.traj.view(frame_index).positions.len();
        let allowed_offsets = supercell_offsets(self.supercell.repeats);
        selection.iter().all(|atom| {
            atom.atom_index < atom_count && allowed_offsets.contains(&atom.image_offset)
        })
    }

    pub(super) fn store_scalars(&mut self, name: String, frame_index: usize, values: Vec<f32>) {
        let frame_count = self.traj.len();
        let entries = self
            .atom_scalars
            .entry(name)
            .or_insert_with(|| vec![None; frame_count]);
        if entries.len() < frame_count {
            entries.resize(frame_count, None);
        }
        entries[frame_index] = Some(values);
    }

    pub(super) fn resize_scalar_storage(&mut self, frame_count: usize) {
        for values in self.atom_scalars.values_mut() {
            values.resize(frame_count, None);
        }
    }

    pub(super) fn sync_main_selection_from_images(&mut self, frame_index: usize) {
        let Some(selected) = self.image_selection.get(frame_index) else {
            return;
        };
        let atom_count = self.traj.view(frame_index).positions.len();
        self.selection.replace(
            frame_index,
            SelectionFrames::mask_from_images(atom_count, selected),
        );
        self.selection.set_order(
            frame_index,
            SelectionFrames::ordered_atoms_from_images(selected),
        );
    }
}

#[derive(Default)]
pub struct CommandOutcome {
    pub should_close: bool,
}
