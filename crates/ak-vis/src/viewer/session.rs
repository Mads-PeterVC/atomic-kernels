use ak_core::{Structure, Trajectory};
use bevy::prelude::{Resource, Vec3};
use smallvec::SmallVec;
use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::ScalarColorMap;

mod camera;
mod selection;

#[cfg(test)]
mod tests;

pub use camera::camera_view_for_frame;

pub enum ViewerCommand {
    LoadTrajectory {
        frames: Vec<Structure>,
        initial_frame: usize,
    },
    AppendFrame {
        frame: Structure,
    },
    SetCurrentFrame {
        index: usize,
    },
    SetFollowTail {
        enabled: bool,
    },
    SetAtomScalars {
        name: String,
        values: Vec<f32>,
        frame_index: Option<usize>,
    },
    ColorByScalar {
        name: String,
        palette: ScalarColorMap,
        min: Option<f32>,
        max: Option<f32>,
        append: bool,
    },
    ResetAtomColors,
    SetBonds {
        bonds: BondList,
        frame_index: Option<usize>,
    },
    AddBonds {
        bonds: BondList,
        frame_index: Option<usize>,
    },
    RemoveBonds {
        bonds: BondList,
        frame_index: Option<usize>,
    },
    ClearBonds {
        frame_index: Option<usize>,
    },
    SetFaces {
        faces: FaceList,
        frame_index: Option<usize>,
    },
    AddFaces {
        faces: FaceList,
        frame_index: Option<usize>,
    },
    RemoveFaces {
        faces: FaceList,
        frame_index: Option<usize>,
    },
    ClearFaces {
        frame_index: Option<usize>,
    },
    SetRenderStyle {
        style: RenderStyle,
        selection: Vec<bool>,
        frame_index: Option<usize>,
        append: bool,
    },
    ResetRenderStyle,
    ReplaceSelection {
        selection: Vec<bool>,
        frame_index: Option<usize>,
    },
    AddSelection {
        selection: Vec<bool>,
        frame_index: Option<usize>,
    },
    RemoveSelection {
        selection: Vec<bool>,
        frame_index: Option<usize>,
    },
    ClearSelection {
        frame_index: Option<usize>,
    },
    ReplaceImageSelection {
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    },
    AddImageSelection {
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    },
    RemoveImageSelection {
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    },
    ClearImageSelection {
        frame_index: Option<usize>,
    },
    SetSupercell {
        repeats: [u32; 3],
    },
    IncrementSupercellAxis {
        axis: usize,
    },
    DecrementSupercellAxis {
        axis: usize,
    },
    ResetSupercell,
    SetGhostRepeatedImages {
        enabled: bool,
    },
    ToggleGhostRepeatedImages,
    SetCameraView {
        focus: Option<[f32; 3]>,
        radius: Option<f32>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    },
    PanCamera {
        delta: [f32; 3],
    },
    ZoomCamera {
        factor: Option<f32>,
        delta: Option<f32>,
    },
    OrbitCamera {
        yaw_delta: f32,
        pitch_delta: f32,
    },
    FrameAll,
    StartOrbit {
        yaw_rate: f32,
        pitch_rate: f32,
    },
    StopCameraMotion,
    Close,
}

#[derive(Clone)]
pub struct ViewerSessionHandle {
    sender: Sender<ViewerCommand>,
    readiness: Arc<ViewerReadiness>,
    snapshot: Arc<Mutex<ViewerSnapshot>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewerLifecycleState {
    Pending,
    Ready,
    Closed,
}

#[derive(Debug)]
pub struct ViewerReadiness {
    state: Mutex<ViewerLifecycleState>,
    changed: Condvar,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionFrames {
    frames: Vec<Vec<bool>>,
    ordered: Vec<Vec<usize>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SelectedImageAtom {
    pub atom_index: usize,
    pub image_offset: [i32; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageSelectionFrames {
    frames: Vec<Vec<SelectedImageAtom>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupercellSettings {
    pub repeats: [u32; 3],
    pub ghost_repeated_images: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewerSnapshot {
    pub current_frame: usize,
    pub selection: SelectionFrames,
    pub image_selection: ImageSelectionFrames,
    pub supercell: SupercellSettings,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewerSessionClosed;

impl std::fmt::Display for ViewerSessionClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "viewer session is no longer available")
    }
}

impl std::error::Error for ViewerSessionClosed {}

#[derive(Clone, Debug, PartialEq)]
pub struct AtomColorRule {
    pub name: String,
    pub palette: ScalarColorMap,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BondList {
    edges: Vec<(usize, usize)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BondFrames {
    frames: Vec<Option<BondList>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub atoms: SmallVec<[usize; 4]>,
    pub color: [f32; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceList {
    faces: Vec<Face>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaceFrames {
    frames: Vec<Option<FaceList>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BondScope {
    BothSelected,
    TouchSelection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BallAndStickStyle {
    pub atom_scale: f32,
    pub bond_radius: f32,
    pub bond_color: [f32; 4],
    pub bond_scope: BondScope,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderStyle {
    SpaceFilling,
    BallAndStick(BallAndStickStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderStyleRule {
    pub frame_index: usize,
    pub selection: Vec<bool>,
    pub style: RenderStyle,
}

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

impl ViewerSessionHandle {
    pub fn new(sender: Sender<ViewerCommand>) -> Self {
        Self {
            sender,
            readiness: Arc::new(ViewerReadiness::new()),
            snapshot: Arc::new(Mutex::new(ViewerSnapshot::default())),
        }
    }

    pub fn with_readiness(sender: Sender<ViewerCommand>, readiness: Arc<ViewerReadiness>) -> Self {
        Self {
            sender,
            readiness,
            snapshot: Arc::new(Mutex::new(ViewerSnapshot::default())),
        }
    }

    pub fn wait_until_ready(&self, timeout: Option<Duration>) -> bool {
        self.readiness.wait(timeout)
    }

    pub fn readiness(&self) -> &Arc<ViewerReadiness> {
        &self.readiness
    }

    pub fn snapshot(&self) -> &Arc<Mutex<ViewerSnapshot>> {
        &self.snapshot
    }

    pub fn load_trajectory(
        &self,
        frames: Vec<Structure>,
        initial_frame: usize,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::LoadTrajectory {
                frames,
                initial_frame,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn append_frame(&self, frame: Structure) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::AppendFrame { frame })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_current_frame(&self, index: usize) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetCurrentFrame { index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_follow_tail(&self, enabled: bool) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetFollowTail { enabled })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_atom_scalars(
        &self,
        name: String,
        values: Vec<f32>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetAtomScalars {
                name,
                values,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn color_by_scalar(
        &self,
        name: String,
        palette: ScalarColorMap,
        min: Option<f32>,
        max: Option<f32>,
        append: bool,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ColorByScalar {
                name,
                palette,
                min,
                max,
                append,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn reset_atom_colors(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ResetAtomColors)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_bonds(
        &self,
        bonds: BondList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetBonds { bonds, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn add_bonds(
        &self,
        bonds: BondList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::AddBonds { bonds, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn remove_bonds(
        &self,
        bonds: BondList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::RemoveBonds { bonds, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn clear_bonds(&self, frame_index: Option<usize>) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ClearBonds { frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_faces(
        &self,
        faces: FaceList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetFaces { faces, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn add_faces(
        &self,
        faces: FaceList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::AddFaces { faces, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn remove_faces(
        &self,
        faces: FaceList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::RemoveFaces { faces, frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn clear_faces(&self, frame_index: Option<usize>) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ClearFaces { frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_render_style(
        &self,
        style: RenderStyle,
        selection: Vec<bool>,
        frame_index: Option<usize>,
        append: bool,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetRenderStyle {
                style,
                selection,
                frame_index,
                append,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn reset_render_style(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ResetRenderStyle)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn replace_selection(
        &self,
        selection: Vec<bool>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ReplaceSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn add_selection(
        &self,
        selection: Vec<bool>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::AddSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn remove_selection(
        &self,
        selection: Vec<bool>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::RemoveSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn clear_selection(&self, frame_index: Option<usize>) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ClearSelection { frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn replace_image_selection(
        &self,
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ReplaceImageSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn add_image_selection(
        &self,
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::AddImageSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn remove_image_selection(
        &self,
        selection: Vec<SelectedImageAtom>,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::RemoveImageSelection {
                selection,
                frame_index,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn clear_image_selection(
        &self,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ClearImageSelection { frame_index })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn selected_atoms(&self, frame_index: Option<usize>) -> Vec<usize> {
        let Ok(snapshot) = self.snapshot.lock() else {
            return Vec::new();
        };
        let target_frame = frame_index.unwrap_or(snapshot.current_frame);
        snapshot.selection.selected_indices(target_frame)
    }

    pub fn selected_images(&self, frame_index: Option<usize>) -> Vec<SelectedImageAtom> {
        let Ok(snapshot) = self.snapshot.lock() else {
            return Vec::new();
        };
        let target_frame = frame_index.unwrap_or(snapshot.current_frame);
        snapshot.image_selection.selected(target_frame)
    }

    pub fn set_supercell(&self, repeats: [u32; 3]) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetSupercell { repeats })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn increment_supercell_axis(&self, axis: usize) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::IncrementSupercellAxis { axis })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn decrement_supercell_axis(&self, axis: usize) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::DecrementSupercellAxis { axis })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn reset_supercell(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ResetSupercell)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn set_ghost_repeated_images(&self, enabled: bool) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetGhostRepeatedImages { enabled })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn toggle_ghost_repeated_images(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ToggleGhostRepeatedImages)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn supercell(&self) -> SupercellSettings {
        let Ok(snapshot) = self.snapshot.lock() else {
            return SupercellSettings::default();
        };
        snapshot.supercell
    }

    pub fn set_camera_view(
        &self,
        focus: Option<[f32; 3]>,
        radius: Option<f32>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetCameraView {
                focus,
                radius,
                yaw,
                pitch,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn pan_camera(&self, delta: [f32; 3]) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::PanCamera { delta })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn zoom_camera(
        &self,
        factor: Option<f32>,
        delta: Option<f32>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ZoomCamera { factor, delta })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn orbit_camera(
        &self,
        yaw_delta: f32,
        pitch_delta: f32,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::OrbitCamera {
                yaw_delta,
                pitch_delta,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn frame_all(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::FrameAll)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn start_orbit(&self, yaw_rate: f32, pitch_rate: f32) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::StartOrbit {
                yaw_rate,
                pitch_rate,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn stop_camera_motion(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::StopCameraMotion)
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn close(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::Close)
            .map_err(|_| ViewerSessionClosed)
    }
}

impl ViewerReadiness {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ViewerLifecycleState::Pending),
            changed: Condvar::new(),
        }
    }

    pub fn mark_ready(&self) {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");
        if *state == ViewerLifecycleState::Pending {
            *state = ViewerLifecycleState::Ready;
            self.changed.notify_all();
        }
    }

    pub fn mark_closed(&self) {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");
        if *state != ViewerLifecycleState::Closed {
            *state = ViewerLifecycleState::Closed;
            self.changed.notify_all();
        }
    }

    pub fn wait(&self, timeout: Option<Duration>) -> bool {
        let mut state = self.state.lock().expect("viewer readiness mutex poisoned");

        match timeout {
            Some(timeout) => {
                let deadline = Instant::now() + timeout;
                while *state == ViewerLifecycleState::Pending {
                    let now = Instant::now();
                    if now >= deadline {
                        return false;
                    }

                    let remaining = deadline.saturating_duration_since(now);
                    let (next_state, result) = self
                        .changed
                        .wait_timeout(state, remaining)
                        .expect("viewer readiness mutex poisoned");
                    state = next_state;
                    if result.timed_out() && *state == ViewerLifecycleState::Pending {
                        return false;
                    }
                }
            }
            None => {
                while *state == ViewerLifecycleState::Pending {
                    state = self
                        .changed
                        .wait(state)
                        .expect("viewer readiness mutex poisoned");
                }
            }
        }

        *state == ViewerLifecycleState::Ready
    }
}

impl Default for ViewerReadiness {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayAtom {
    pub identity: SelectedImageAtom,
    pub position: [f64; 3],
    pub is_main_cell: bool,
}

#[derive(Resource)]
pub struct ViewerState {
    pub traj: Trajectory,
    pub current: usize,
    pub follow_tail: bool,
    pub atom_scalars: HashMap<String, Vec<Option<Vec<f32>>>>,
    pub atom_color_rules: Vec<AtomColorRule>,
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
            atom_color_rules: Vec::new(),
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
                    self.current = self.traj.len() - 1;
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
                    && self.atom_color_rules.iter().any(|rule| rule.name == name);
                if self.validate_scalar_values(target_frame, &values) {
                    self.store_scalars(name, target_frame, values);
                    self.needs_render = should_refresh_active_mode;
                }
                CommandOutcome::default()
            }
            ViewerCommand::ColorByScalar {
                name,
                palette,
                min,
                max,
                append,
            } => {
                let rule = AtomColorRule {
                    name,
                    palette,
                    min,
                    max,
                };
                if append {
                    self.atom_color_rules.push(rule);
                } else {
                    self.atom_color_rules.clear();
                    self.atom_color_rules.push(rule);
                }
                self.needs_render = true;
                CommandOutcome::default()
            }
            ViewerCommand::ResetAtomColors => {
                self.atom_color_rules.clear();
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

    fn cell_changed(&self, old_index: usize, new_index: usize) -> bool {
        self.traj.view(old_index).cell != self.traj.view(new_index).cell
    }

    fn validate_scalar_values(&self, frame_index: usize, values: &[f32]) -> bool {
        frame_index < self.traj.len() && self.traj.view(frame_index).positions.len() == values.len()
    }

    fn validate_selection(&self, frame_index: usize, selection: &[bool]) -> bool {
        frame_index < self.traj.len()
            && self.traj.view(frame_index).positions.len() == selection.len()
    }

    fn validate_image_selection(
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

    fn store_scalars(&mut self, name: String, frame_index: usize, values: Vec<f32>) {
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

    fn resize_scalar_storage(&mut self, frame_count: usize) {
        for values in self.atom_scalars.values_mut() {
            values.resize(frame_count, None);
        }
    }

    fn sync_main_selection_from_images(&mut self, frame_index: usize) {
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

impl BondList {
    pub fn new(edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut canonical = BTreeSet::new();
        for (i, j) in edges {
            if i == j {
                continue;
            }
            canonical.insert((i.min(j), i.max(j)));
        }
        Self {
            edges: canonical.into_iter().collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.edges.iter()
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn merged(&self, other: &BondList) -> Self {
        Self::new(self.iter().copied().chain(other.iter().copied()))
    }

    pub fn without(&self, other: &BondList) -> Self {
        let removals: BTreeSet<_> = other.iter().copied().collect();
        Self::new(self.iter().copied().filter(|edge| !removals.contains(edge)))
    }
}

impl BondFrames {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frames: vec![None; frame_count],
        }
    }

    pub fn set(&mut self, frame_index: usize, bonds: BondList) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = Some(bonds);
        }
    }

    pub fn add(&mut self, frame_index: usize, bonds: BondList) {
        if frame_index >= self.frames.len() {
            return;
        }
        let merged = match self.frames[frame_index].take() {
            Some(existing) => existing.merged(&bonds),
            None => bonds,
        };
        self.frames[frame_index] = Some(merged);
    }

    pub fn remove(&mut self, frame_index: usize, bonds: &BondList) {
        if frame_index >= self.frames.len() {
            return;
        }
        if let Some(existing) = self.frames[frame_index].take() {
            let updated = existing.without(bonds);
            self.frames[frame_index] = (!updated.is_empty()).then_some(updated);
        }
    }

    pub fn clear(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = None;
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&BondList> {
        self.frames
            .get(frame_index)
            .and_then(|bonds| bonds.as_ref())
    }

    pub fn resize(&mut self, frame_count: usize) {
        self.frames.resize(frame_count, None);
    }
}

impl Face {
    pub fn new(atoms: impl IntoIterator<Item = usize>, color: [f32; 4]) -> Option<Self> {
        let atoms: SmallVec<[usize; 4]> = atoms.into_iter().collect();
        if atoms.len() < 3 {
            return None;
        }
        let unique: BTreeSet<_> = atoms.iter().copied().collect();
        if unique.len() != atoms.len() {
            return None;
        }
        Some(Self { atoms, color })
    }

    pub fn color(self) -> bevy::color::Color {
        bevy::color::Color::srgba(self.color[0], self.color[1], self.color[2], self.color[3])
    }

    fn canonical_atoms(&self) -> Vec<usize> {
        canonicalize_face_atoms(&self.atoms)
    }
}

impl FaceList {
    pub fn new(faces: impl IntoIterator<Item = Face>) -> Self {
        let mut seen = BTreeSet::new();
        let mut canonical = Vec::new();

        for face in faces {
            let key = face.canonical_atoms();
            if seen.insert(key) {
                canonical.push(face);
            }
        }

        Self { faces: canonical }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter()
    }

    pub fn len(&self) -> usize {
        self.faces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    pub fn validated_for_atom_count(&self, atom_count: usize) -> Self {
        Self::new(
            self.faces
                .iter()
                .filter(|face| face.atoms.iter().all(|&index| index < atom_count))
                .cloned(),
        )
    }

    pub fn merged(&self, other: &FaceList) -> Self {
        Self::new(self.iter().cloned().chain(other.iter().cloned()))
    }

    pub fn without(&self, other: &FaceList) -> Self {
        let removals: BTreeSet<_> = other.iter().map(Face::canonical_atoms).collect();
        Self::new(
            self.faces
                .iter()
                .filter(|face| !removals.contains(&face.canonical_atoms()))
                .cloned(),
        )
    }
}

impl FaceFrames {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frames: vec![None; frame_count],
        }
    }

    pub fn set(&mut self, frame_index: usize, faces: FaceList) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = Some(faces);
        }
    }

    pub fn add(&mut self, frame_index: usize, faces: FaceList) {
        if frame_index >= self.frames.len() {
            return;
        }
        let merged = match self.frames[frame_index].take() {
            Some(existing) => existing.merged(&faces),
            None => faces,
        };
        self.frames[frame_index] = Some(merged);
    }

    pub fn remove(&mut self, frame_index: usize, faces: &FaceList) {
        if frame_index >= self.frames.len() {
            return;
        }
        if let Some(existing) = self.frames[frame_index].take() {
            let updated = existing.without(faces);
            self.frames[frame_index] = (!updated.is_empty()).then_some(updated);
        }
    }

    pub fn clear(&mut self, frame_index: usize) {
        if frame_index < self.frames.len() {
            self.frames[frame_index] = None;
        }
    }

    pub fn get(&self, frame_index: usize) -> Option<&FaceList> {
        self.frames
            .get(frame_index)
            .and_then(|faces| faces.as_ref())
    }

    pub fn resize(&mut self, frame_count: usize) {
        self.frames.resize(frame_count, None);
    }
}

impl BallAndStickStyle {
    pub fn bond_color(self) -> bevy::color::Color {
        bevy::color::Color::srgba(
            self.bond_color[0],
            self.bond_color[1],
            self.bond_color[2],
            self.bond_color[3],
        )
    }
}

#[derive(Default)]
pub struct CommandOutcome {
    pub should_close: bool,
}

impl Default for ViewerSnapshot {
    fn default() -> Self {
        Self {
            current_frame: 0,
            selection: SelectionFrames {
                frames: Vec::new(),
                ordered: Vec::new(),
            },
            image_selection: ImageSelectionFrames { frames: Vec::new() },
            supercell: SupercellSettings::default(),
        }
    }
}

impl Default for SupercellSettings {
    fn default() -> Self {
        Self {
            repeats: [0, 0, 0],
            ghost_repeated_images: true,
        }
    }
}

fn clamp_frame(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { index.min(len - 1) }
}

fn axis_offsets(repeat_extent: u32) -> Vec<i32> {
    let repeat_extent = repeat_extent as i32;
    (-repeat_extent..=repeat_extent).collect()
}

fn supercell_offsets(repeats: [u32; 3]) -> Vec<[i32; 3]> {
    let mut offsets = Vec::new();
    for ia in axis_offsets(repeats[0]) {
        for ib in axis_offsets(repeats[1]) {
            for ic in axis_offsets(repeats[2]) {
                offsets.push([ia, ib, ic]);
            }
        }
    }
    offsets.sort_by_key(|offset| {
        (
            offset.iter().map(|value| value.abs()).sum::<i32>(),
            offset[0].abs(),
            offset[1].abs(),
            offset[2].abs(),
            offset[0],
            offset[1],
            offset[2],
        )
    });
    offsets
}

fn scaled_cell_translation(cell: ak_core::geometry::Cell, offset: [i32; 3]) -> [f64; 3] {
    let a = cell.a();
    let b = cell.b();
    let c = cell.c();
    let shift = a * offset[0] as f64 + b * offset[1] as f64 + c * offset[2] as f64;
    [shift[0], shift[1], shift[2]]
}

fn canonicalize_face_atoms(atoms: &[usize]) -> Vec<usize> {
    let forward = minimum_face_rotation(atoms);
    let mut reversed = atoms.to_vec();
    reversed.reverse();
    let reversed = minimum_face_rotation(&reversed);
    if reversed < forward {
        reversed
    } else {
        forward
    }
}

fn minimum_face_rotation(atoms: &[usize]) -> Vec<usize> {
    let mut best = atoms.to_vec();
    for shift in 1..atoms.len() {
        let rotated: Vec<usize> = atoms
            .iter()
            .cycle()
            .skip(shift)
            .take(atoms.len())
            .copied()
            .collect();
        if rotated < best {
            best = rotated;
        }
    }
    best
}
