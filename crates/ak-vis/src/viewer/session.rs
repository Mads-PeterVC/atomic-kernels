use ak_core::{Structure, Trajectory};
use bevy::prelude::Resource;
use std::sync::mpsc::Sender;

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
    Close,
}

#[derive(Clone)]
pub struct ViewerSessionHandle {
    sender: Sender<ViewerCommand>,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewerSessionClosed;

impl std::fmt::Display for ViewerSessionClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "viewer session is no longer available")
    }
}

impl std::error::Error for ViewerSessionClosed {}

impl ViewerSessionHandle {
    pub fn new(sender: Sender<ViewerCommand>) -> Self {
        Self { sender }
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

    pub fn close(&self) -> Result<(), ViewerSessionClosed> {
        self.sender
            .send(ViewerCommand::Close)
            .map_err(|_| ViewerSessionClosed)
    }
}

#[derive(Resource)]
pub struct ViewerState {
    pub traj: Trajectory,
    pub current: usize,
    pub follow_tail: bool,
    pub needs_render: bool,
    pub needs_camera_reset: bool,
}

impl ViewerState {
    pub fn new(traj: Trajectory, initial_frame: usize) -> Self {
        let current = clamp_frame(initial_frame, traj.len());
        Self {
            traj,
            current,
            follow_tail: false,
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
                self.traj = Trajectory::new(frames);
                self.current = clamp_frame(initial_frame, self.traj.len());
                self.needs_render = true;
                self.needs_camera_reset = true;
                CommandOutcome::default()
            }
            ViewerCommand::AppendFrame { frame } => {
                let was_empty = self.traj.is_empty();
                let previous = if was_empty { None } else { Some(self.current) };
                self.traj.append(frame);
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
}

#[derive(Default)]
pub struct CommandOutcome {
    pub should_close: bool,
}

fn clamp_frame(index: usize, len: usize) -> usize {
    if len == 0 { 0 } else { index.min(len - 1) }
}

#[cfg(test)]
mod tests {
    use super::{ViewerCommand, ViewerState};
    use ak_core::{Structure, Trajectory};

    fn test_structure(x: f64) -> Structure {
        Structure::new(
            vec![[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0]],
            vec![1, 1],
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
}
