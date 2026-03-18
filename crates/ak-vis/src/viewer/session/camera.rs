use bevy::prelude::Vec3;

use crate::structure_vec3_to_world;

use super::{CameraState, CameraView, OrbitMotion, ViewerCommand, ViewerState};

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
