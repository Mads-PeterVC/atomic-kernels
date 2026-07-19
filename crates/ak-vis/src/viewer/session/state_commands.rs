use ak_core::Trajectory;

use super::snapshot::clamp_frame;
use super::{
    AtomAppearanceRule, BondFrames, CommandOutcome, FaceFrames, ImageSelectionFrames,
    RenderStyleRule, SelectionFrames, SupercellSettings, ViewerCommand, ViewerState,
};

impl ViewerState {
    pub fn apply_command(&mut self, command: ViewerCommand) -> CommandOutcome {
        match command {
            ViewerCommand::LoadTrajectory {
                frames,
                initial_frame,
            } => {
                let frame_count = frames.len();
                self.traj = Trajectory::new(frames);
                self.current = clamp_frame(initial_frame, self.traj.len());
                self.resize_scalar_storage(frame_count);
                self.bonds = BondFrames::new(frame_count);
                self.faces = FaceFrames::new(frame_count);
                self.render_style_rules.clear();
                self.selection = SelectionFrames::new(&self.traj);
                self.image_selection = ImageSelectionFrames::new(frame_count);
                self.supercell = SupercellSettings::default();
                self.needs_render = true;
                self.needs_camera_reset = true;
                CommandOutcome::default()
            }
            ViewerCommand::AppendFrame { frame } => {
                let was_empty = self.traj.is_empty();
                let previous = if was_empty { None } else { Some(self.current) };
                self.traj.append(frame);
                self.bonds.resize(self.traj.len());
                self.faces.resize(self.traj.len());
                self.selection.append_empty_for_atom_count(
                    self.traj.view(self.traj.len() - 1).positions.len(),
                );
                self.image_selection.append_empty_frame();
                if was_empty {
                    self.current = 0;
                    self.needs_render = true;
                    self.needs_camera_reset = true;
                } else if self.follow_tail {
                    let next = self.traj.len() - 1;
                    if let Some(previous) = previous {
                        self.copy_selection_between_compatible_frames(previous, next);
                    }
                    self.current = next;
                    self.needs_render = true;
                    self.needs_camera_reset = previous
                        .map(|old| self.cell_changed(old, self.current))
                        .unwrap_or(false);
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetCurrentFrame { index } => {
                if index < self.traj.len() && index != self.current {
                    let previous = self.current;
                    self.copy_selection_between_compatible_frames(previous, index);
                    self.current = index;
                    self.needs_render = true;
                    self.needs_camera_reset = self.cell_changed(previous, self.current);
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetFollowTail { enabled } => {
                self.follow_tail = enabled;
                CommandOutcome::default()
            }
            ViewerCommand::SetAtomScalars {
                name,
                values,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                let should_refresh_active_mode = self.current == target_frame
                    && self
                        .atom_appearance_rules
                        .iter()
                        .any(|rule| rule.name == name);
                if self.validate_scalar_values(target_frame, &values) {
                    self.store_scalars(name, target_frame, values);
                    self.needs_render = should_refresh_active_mode;
                }
                CommandOutcome::default()
            }
            ViewerCommand::MapAppearanceByScalar {
                name,
                channel,
                palette,
                min,
                max,
                append,
            } => {
                let rule = AtomAppearanceRule {
                    name,
                    channel,
                    palette,
                    min,
                    max,
                };
                if append {
                    self.atom_appearance_rules.push(rule);
                } else {
                    self.atom_appearance_rules
                        .retain(|existing| existing.channel != channel);
                    self.atom_appearance_rules.push(rule);
                }
                self.needs_render = true;
                CommandOutcome::default()
            }
            ViewerCommand::ResetAtomAppearance { channel } => {
                match channel {
                    Some(channel) => {
                        self.atom_appearance_rules
                            .retain(|rule| rule.channel != channel);
                    }
                    None => self.atom_appearance_rules.clear(),
                }
                self.needs_render = true;
                CommandOutcome::default()
            }
            ViewerCommand::SetBonds { bonds, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.bonds.set(target_frame, bonds);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::AddBonds { bonds, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.bonds.add(target_frame, bonds);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::RemoveBonds { bonds, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.bonds.remove(target_frame, &bonds);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ClearBonds { frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.bonds.clear(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetFaces { faces, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    let atom_count = self.traj.view(target_frame).positions.len();
                    self.faces
                        .set(target_frame, faces.validated_for_atom_count(atom_count));
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::AddFaces { faces, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    let atom_count = self.traj.view(target_frame).positions.len();
                    self.faces
                        .add(target_frame, faces.validated_for_atom_count(atom_count));
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::RemoveFaces { faces, frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    let atom_count = self.traj.view(target_frame).positions.len();
                    self.faces
                        .remove(target_frame, &faces.validated_for_atom_count(atom_count));
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ClearFaces { frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.faces.clear(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetRenderStyle {
                style,
                selection,
                frame_index,
                append,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_selection(target_frame, &selection) {
                    let rule = RenderStyleRule {
                        frame_index: target_frame,
                        selection,
                        style,
                    };
                    if append {
                        self.render_style_rules.push(rule);
                    } else {
                        self.render_style_rules.clear();
                        self.render_style_rules.push(rule);
                    }
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ResetRenderStyle => {
                self.render_style_rules.clear();
                self.needs_render = true;
                CommandOutcome::default()
            }
            ViewerCommand::ReplaceSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_selection(target_frame, &selection) {
                    self.selection.replace(target_frame, selection);
                    self.image_selection.replace(
                        target_frame,
                        self.selection.selected_main_images(target_frame),
                    );
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::AddSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_selection(target_frame, &selection) {
                    self.selection.add(target_frame, &selection);
                    self.image_selection.replace(
                        target_frame,
                        self.selection.selected_main_images(target_frame),
                    );
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::RemoveSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_selection(target_frame, &selection) {
                    self.selection.remove(target_frame, &selection);
                    self.image_selection.replace(
                        target_frame,
                        self.selection.selected_main_images(target_frame),
                    );
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ClearSelection { frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.selection.clear(target_frame);
                    self.image_selection.clear(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ReplaceImageSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_image_selection(target_frame, &selection) {
                    self.image_selection.replace(target_frame, selection);
                    self.sync_main_selection_from_images(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::AddImageSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_image_selection(target_frame, &selection) {
                    self.image_selection.add(target_frame, &selection);
                    self.sync_main_selection_from_images(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::RemoveImageSelection {
                selection,
                frame_index,
            } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if self.validate_image_selection(target_frame, &selection) {
                    self.image_selection.remove(target_frame, &selection);
                    self.sync_main_selection_from_images(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ClearImageSelection { frame_index } => {
                let target_frame = frame_index.unwrap_or(self.current);
                if target_frame < self.traj.len() {
                    self.image_selection.clear(target_frame);
                    self.sync_main_selection_from_images(target_frame);
                    self.needs_render = self.current == target_frame;
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetSupercell { repeats } => {
                if self.supercell.repeats != repeats {
                    self.supercell.repeats = repeats;
                    self.needs_render = true;
                }
                CommandOutcome::default()
            }
            ViewerCommand::IncrementSupercellAxis { axis } => {
                if axis < 3 {
                    self.supercell.repeats[axis] = self.supercell.repeats[axis].saturating_add(1);
                    self.needs_render = true;
                }
                CommandOutcome::default()
            }
            ViewerCommand::DecrementSupercellAxis { axis } => {
                if axis < 3 {
                    let next = self.supercell.repeats[axis].saturating_sub(1);
                    if self.supercell.repeats[axis] != next {
                        self.supercell.repeats[axis] = next;
                        self.needs_render = true;
                    }
                }
                CommandOutcome::default()
            }
            ViewerCommand::ResetSupercell => {
                if self.supercell.repeats != [0, 0, 0] {
                    self.supercell.repeats = [0, 0, 0];
                    self.needs_render = true;
                }
                CommandOutcome::default()
            }
            ViewerCommand::SetGhostRepeatedImages { enabled } => {
                if self.supercell.ghost_repeated_images != enabled {
                    self.supercell.ghost_repeated_images = enabled;
                    self.needs_render = true;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ToggleGhostRepeatedImages => {
                self.supercell.ghost_repeated_images = !self.supercell.ghost_repeated_images;
                self.needs_render = true;
                CommandOutcome::default()
            }
            ViewerCommand::SetCameraView { .. }
            | ViewerCommand::PanCamera { .. }
            | ViewerCommand::ZoomCamera { .. }
            | ViewerCommand::OrbitCamera { .. }
            | ViewerCommand::FrameAll
            | ViewerCommand::StartOrbit { .. }
            | ViewerCommand::StopCameraMotion => CommandOutcome::default(),
            ViewerCommand::Close => CommandOutcome { should_close: true },
        }
    }
}
