use ak_core::Structure;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::ScalarColorMap;

use super::{
    AppearanceChannel, BondList, FaceList, RenderStyle, SelectedImageAtom, SupercellSettings,
    ViewerCommand, ViewerReadiness, ViewerSessionClosed, ViewerSnapshot,
};

#[derive(Clone)]
pub struct ViewerSessionHandle {
    sender: Sender<ViewerCommand>,
    readiness: Arc<ViewerReadiness>,
    snapshot: Arc<Mutex<ViewerSnapshot>>,
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

    pub fn map_appearance_by_scalar(
        &self,
        name: String,
        channel: AppearanceChannel,
        palette: Option<ScalarColorMap>,
        min: Option<f32>,
        max: Option<f32>,
        append: bool,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::MapAppearanceByScalar {
                name,
                channel,
                palette,
                min,
                max,
                append,
            })
            .map_err(|_| ViewerSessionClosed)
    }

    pub fn reset_atom_appearance(
        &self,
        channel: Option<AppearanceChannel>,
    ) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::ResetAtomAppearance { channel })
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
