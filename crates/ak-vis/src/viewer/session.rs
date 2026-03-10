use ak_core::{Structure, Trajectory};
use bevy::prelude::{Resource, Vec3};
use smallvec::SmallVec;
use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::ScalarColorMap;
use crate::structure_vec3_to_world;

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
    SetFaces {
        faces: FaceList,
        frame_index: Option<usize>,
    },
    SetRenderStyle {
        style: RenderStyle,
        selection: Vec<bool>,
        frame_index: Option<usize>,
        append: bool,
    },
    ResetRenderStyle,
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
        }
    }

    pub fn with_readiness(sender: Sender<ViewerCommand>, readiness: Arc<ViewerReadiness>) -> Self {
        Self { sender, readiness }
    }

    pub fn wait_until_ready(&self, timeout: Option<Duration>) -> bool {
        self.readiness.wait(timeout)
    }

    pub fn readiness(&self) -> &Arc<ViewerReadiness> {
        &self.readiness
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

    pub fn set_faces(
        &self,
        faces: FaceList,
        frame_index: Option<usize>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::SetFaces { faces, frame_index })
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
    pub needs_render: bool,
    pub needs_camera_reset: bool,
}

impl ViewerState {
    pub fn new(traj: Trajectory, initial_frame: usize) -> Self {
        let frame_count = traj.len();
        let current = clamp_frame(initial_frame, traj.len());
        Self {
            traj,
            current,
            follow_tail: false,
            atom_scalars: HashMap::new(),
            atom_color_rules: Vec::new(),
            bonds: BondFrames::new(frame_count),
            faces: FaceFrames::new(frame_count),
            render_style_rules: Vec::new(),
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

        let previous = self.current;
        self.current = next;
        self.needs_render = true;
        self.needs_camera_reset = self.cell_changed(previous, self.current);
        true
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
        Self::new(self.faces.iter().filter_map(|face| {
            face.atoms
                .iter()
                .all(|&index| index < atom_count)
                .then(|| face.clone())
        }))
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

impl CameraState {
    pub fn new(viewer: &ViewerState) -> Self {
        Self::from_view(camera_view_for_frame(viewer).unwrap_or(CameraView {
            focus: Vec3::ZERO,
            radius: 1.0,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.0,
        }))
    }

    pub fn from_view(view: CameraView) -> Self {
        Self {
            focus: view.focus,
            radius: view.radius,
            yaw: view.yaw,
            pitch: view.pitch,
            needs_apply: true,
            motion: None,
        }
    }

    pub fn apply_command(&mut self, viewer: &ViewerState, command: &ViewerCommand) {
        match command {
            ViewerCommand::SetCameraView {
                focus,
                radius,
                yaw,
                pitch,
            } => {
                self.motion = None;
                if let Some(focus) = focus {
                    self.focus = Vec3::from_array(*focus);
                }
                if let Some(radius) = radius {
                    self.radius = (*radius).max(f32::EPSILON);
                }
                if let Some(yaw) = yaw {
                    self.yaw = *yaw;
                }
                if let Some(pitch) = pitch {
                    self.pitch = *pitch;
                }
                self.needs_apply = true;
            }
            ViewerCommand::PanCamera { delta } => {
                self.motion = None;
                self.focus += Vec3::from_array(*delta);
                self.needs_apply = true;
            }
            ViewerCommand::ZoomCamera { factor, delta } => {
                self.motion = None;
                if let Some(factor) = factor {
                    self.radius = (self.radius * *factor).max(f32::EPSILON);
                }
                if let Some(delta) = delta {
                    self.radius = (self.radius + *delta).max(f32::EPSILON);
                }
                self.needs_apply = true;
            }
            ViewerCommand::OrbitCamera {
                yaw_delta,
                pitch_delta,
            } => {
                self.motion = None;
                self.yaw += *yaw_delta;
                self.pitch += *pitch_delta;
                self.needs_apply = true;
            }
            ViewerCommand::FrameAll => {
                if let Some(view) = camera_view_for_frame(viewer) {
                    *self = Self::from_view(view);
                }
            }
            ViewerCommand::StartOrbit {
                yaw_rate,
                pitch_rate,
            } => {
                self.motion = Some(OrbitMotion {
                    yaw_rate: *yaw_rate,
                    pitch_rate: *pitch_rate,
                });
            }
            ViewerCommand::StopCameraMotion => {
                self.motion = None;
            }
            _ => {}
        }
    }

    pub fn reset_for_frame(&mut self, viewer: &ViewerState) {
        if let Some(view) = camera_view_for_frame(viewer) {
            *self = Self::from_view(view);
        }
    }

    pub fn tick_motion(&mut self, delta_seconds: f32) {
        if let Some(motion) = self.motion {
            self.yaw += motion.yaw_rate * delta_seconds;
            self.pitch += motion.pitch_rate * delta_seconds;
            self.needs_apply = true;
        }
    }
}

#[derive(Default)]
pub struct CommandOutcome {
    pub should_close: bool,
}

fn clamp_frame(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { index.min(len - 1) }
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

pub fn camera_view_for_frame(viewer: &ViewerState) -> Option<CameraView> {
    if !viewer.has_frames() {
        return None;
    }

    let view = viewer.traj.view(viewer.current);
    let focus = structure_vec3_to_world(Vec3::from_slice(
        view.cell.reduced(0.5, 0.5, 0.5).cast::<f32>().as_slice(),
    ));
    let radius = [view.cell.a(), view.cell.b(), view.cell.c()]
        .iter()
        .fold(0.0_f64, |acc, v| acc.max(v.norm())) as f32;

    Some(CameraView {
        focus,
        radius: 2.5 * radius.max(f32::EPSILON),
        yaw: -std::f32::consts::FRAC_PI_2,
        pitch: 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        AtomColorRule, BallAndStickStyle, BondFrames, BondList, BondScope, CameraState, Face,
        FaceFrames, FaceList, RenderStyle, ScalarColorMap, ViewerCommand, ViewerState,
        camera_view_for_frame,
    };
    use ak_core::{Structure, Trajectory};
    use bevy::prelude::Vec3;

    fn test_structure(x: f64) -> Structure {
        Structure::new(
            vec![[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0]],
            vec![1, 1],
            [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
            [false, false, false],
        )
    }

    fn test_structure4() -> Structure {
        Structure::new(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            vec![1, 1, 1, 1],
            [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
            [false, false, false],
        )
    }

    #[test]
    fn load_trajectory_sets_initial_frame() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        state.needs_render = false;
        state.needs_camera_reset = false;

        state.apply_command(ViewerCommand::LoadTrajectory {
            frames: vec![
                test_structure(0.0),
                test_structure(1.0),
                test_structure(2.0),
            ],
            initial_frame: 2,
        });

        assert_eq!(state.current, 2);
        assert_eq!(state.trajectory_len(), 3);
        assert!(state.needs_render);
        assert!(state.needs_camera_reset);
    }

    #[test]
    fn append_frame_increases_trajectory_length() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

        state.apply_command(ViewerCommand::AppendFrame {
            frame: test_structure(1.0),
        });

        assert_eq!(state.trajectory_len(), 2);
    }

    #[test]
    fn follow_tail_on_moves_to_latest_frame_after_append() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        state.apply_command(ViewerCommand::SetFollowTail { enabled: true });

        state.apply_command(ViewerCommand::AppendFrame {
            frame: test_structure(1.0),
        });

        assert_eq!(state.current, 1);
        assert!(state.needs_render);
    }

    #[test]
    fn follow_tail_off_preserves_current_frame_after_append() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        state.needs_render = false;

        state.apply_command(ViewerCommand::AppendFrame {
            frame: test_structure(1.0),
        });

        assert_eq!(state.current, 0);
        assert_eq!(state.trajectory_len(), 2);
        assert!(!state.needs_render);
    }

    #[test]
    fn out_of_bounds_current_frame_is_ignored() {
        let mut state = ViewerState::new(
            Trajectory::new(vec![test_structure(0.0), test_structure(1.0)]),
            0,
        );
        state.needs_render = false;

        state.apply_command(ViewerCommand::SetCurrentFrame { index: 5 });

        assert_eq!(state.current, 0);
        assert!(!state.needs_render);
    }

    #[test]
    fn set_atom_scalars_stores_current_frame_values() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

        state.apply_command(ViewerCommand::SetAtomScalars {
            name: "energy".to_string(),
            values: vec![1.0, 2.0],
            frame_index: None,
        });

        assert_eq!(
            state.atom_scalars["energy"][0].as_ref().unwrap(),
            &vec![1.0, 2.0]
        );
    }

    #[test]
    fn color_by_scalar_switches_color_mode() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

        state.apply_command(ViewerCommand::ColorByScalar {
            name: "energy".to_string(),
            palette: ScalarColorMap::Viridis,
            min: None,
            max: None,
            append: false,
        });

        assert_eq!(
            state.atom_color_rules,
            vec![AtomColorRule {
                name: "energy".to_string(),
                palette: ScalarColorMap::Viridis,
                min: None,
                max: None,
            }]
        );
    }

    #[test]
    fn append_color_rule_preserves_existing_rules() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

        state.apply_command(ViewerCommand::ColorByScalar {
            name: "energy".to_string(),
            palette: ScalarColorMap::Viridis,
            min: None,
            max: None,
            append: false,
        });
        state.apply_command(ViewerCommand::ColorByScalar {
            name: "charge".to_string(),
            palette: ScalarColorMap::Plasma,
            min: Some(-1.0),
            max: Some(1.0),
            append: true,
        });

        assert_eq!(state.atom_color_rules.len(), 2);
        assert_eq!(state.atom_color_rules[0].name, "energy");
        assert_eq!(state.atom_color_rules[1].name, "charge");
    }

    #[test]
    fn bond_list_canonicalizes_edges() {
        let bonds = BondList::new([(2, 1), (1, 2), (0, 0), (0, 3)]);

        let edges: Vec<(usize, usize)> = bonds.iter().copied().collect();
        assert_eq!(edges, vec![(0, 3), (1, 2)]);
    }

    #[test]
    fn bond_frames_store_per_frame_bonds() {
        let mut frames = BondFrames::new(2);
        frames.set(1, BondList::new([(0, 1)]));

        assert!(frames.get(0).is_none());
        assert_eq!(
            frames.get(1).unwrap().iter().copied().collect::<Vec<_>>(),
            vec![(0, 1)]
        );
    }

    #[test]
    fn face_list_canonicalizes_rotations_and_reversals() {
        let faces = FaceList::new([
            Face::new([0, 1, 2, 3], [1.0, 0.0, 0.0, 0.5]).unwrap(),
            Face::new([2, 3, 0, 1], [0.0, 1.0, 0.0, 0.5]).unwrap(),
            Face::new([3, 2, 1, 0], [0.0, 0.0, 1.0, 0.5]).unwrap(),
        ]);

        assert_eq!(faces.len(), 1);
        assert_eq!(faces.iter().next().unwrap().atoms.as_slice(), &[0, 1, 2, 3]);
    }

    #[test]
    fn face_frames_store_per_frame_faces() {
        let mut frames = FaceFrames::new(2);
        frames.set(
            1,
            FaceList::new([Face::new([0, 1, 2], [0.2, 0.4, 0.6, 0.3]).unwrap()]),
        );

        assert!(frames.get(0).is_none());
        assert_eq!(frames.get(1).unwrap().len(), 1);
    }

    #[test]
    fn set_faces_drops_out_of_range_polygons() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure4()]), 0);

        state.apply_command(ViewerCommand::SetFaces {
            faces: FaceList::new([
                Face::new([0, 1, 2], [0.1, 0.2, 0.3, 0.4]).unwrap(),
                Face::new([0, 1, 9], [0.5, 0.6, 0.7, 0.8]).unwrap(),
            ]),
            frame_index: None,
        });

        let faces = state.faces.get(0).unwrap();
        assert_eq!(faces.len(), 1);
        assert_eq!(faces.iter().next().unwrap().atoms.as_slice(), &[0, 1, 2]);
    }

    #[test]
    fn set_render_style_stores_selection_rule() {
        let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

        state.apply_command(ViewerCommand::SetRenderStyle {
            style: RenderStyle::BallAndStick(BallAndStickStyle {
                atom_scale: 0.45,
                bond_radius: 0.08,
                bond_color: [0.7, 0.7, 0.7, 1.0],
                bond_scope: BondScope::TouchSelection,
            }),
            selection: vec![true, false],
            frame_index: None,
            append: false,
        });

        assert_eq!(state.render_style_rules.len(), 1);
        assert_eq!(state.render_style_rules[0].selection, vec![true, false]);
        assert_eq!(state.render_style_rules[0].frame_index, 0);
        assert!(matches!(
            state.render_style_rules[0].style,
            RenderStyle::BallAndStick(BallAndStickStyle {
                bond_scope: BondScope::TouchSelection,
                ..
            })
        ));
    }

    #[test]
    fn set_camera_view_updates_requested_fields() {
        let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        let mut camera = CameraState::new(&viewer);

        camera.apply_command(
            &viewer,
            &ViewerCommand::SetCameraView {
                focus: Some([1.0, 2.0, 3.0]),
                radius: Some(9.0),
                yaw: Some(0.5),
                pitch: Some(-0.25),
            },
        );

        assert_eq!(camera.focus, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(camera.radius, 9.0);
        assert_eq!(camera.yaw, 0.5);
        assert_eq!(camera.pitch, -0.25);
    }

    #[test]
    fn orbit_and_zoom_camera_are_incremental() {
        let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        let mut camera = CameraState::new(&viewer);
        let initial = camera.clone();

        camera.apply_command(
            &viewer,
            &ViewerCommand::OrbitCamera {
                yaw_delta: 0.2,
                pitch_delta: -0.1,
            },
        );
        camera.apply_command(
            &viewer,
            &ViewerCommand::ZoomCamera {
                factor: Some(0.5),
                delta: None,
            },
        );

        assert_eq!(camera.yaw, initial.yaw + 0.2);
        assert_eq!(camera.pitch, initial.pitch - 0.1);
        assert_eq!(camera.radius, initial.radius * 0.5);
    }

    #[test]
    fn start_and_stop_orbit_motion_updates_camera() {
        let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        let mut camera = CameraState::new(&viewer);
        let initial_yaw = camera.yaw;

        camera.apply_command(
            &viewer,
            &ViewerCommand::StartOrbit {
                yaw_rate: 1.0,
                pitch_rate: 0.0,
            },
        );
        camera.tick_motion(0.5);
        assert_eq!(camera.yaw, initial_yaw + 0.5);

        camera.apply_command(&viewer, &ViewerCommand::StopCameraMotion);
        camera.tick_motion(0.5);
        assert_eq!(camera.yaw, initial_yaw + 0.5);
    }

    #[test]
    fn frame_all_restores_default_camera_view() {
        let viewer = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
        let mut camera = CameraState::new(&viewer);
        camera.focus = Vec3::splat(5.0);
        camera.radius = 99.0;
        camera.yaw = 2.0;
        camera.pitch = 1.0;

        camera.apply_command(&viewer, &ViewerCommand::FrameAll);

        let expected = camera_view_for_frame(&viewer).unwrap();
        assert_eq!(camera.focus, expected.focus);
        assert_eq!(camera.radius, expected.radius);
        assert_eq!(camera.yaw, expected.yaw);
        assert_eq!(camera.pitch, expected.pitch);
    }
}
