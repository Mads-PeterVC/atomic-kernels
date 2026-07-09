use ak_core::Structure;

use crate::ScalarColorMap;

use super::{AppearanceChannel, BondList, FaceList, RenderStyle, SelectedImageAtom};

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
    MapAppearanceByScalar {
        name: String,
        channel: AppearanceChannel,
        palette: Option<ScalarColorMap>,
        min: Option<f32>,
        max: Option<f32>,
        append: bool,
    },
    ResetAtomAppearance {
        channel: Option<AppearanceChannel>,
    },
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
