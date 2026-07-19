use super::helpers::*;
use crate::viewer::session::{SelectedImageAtom, ViewerCommand, ViewerState};
use ak_core::Trajectory;
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
fn follow_tail_append_copies_selection_to_compatible_frame() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    state.apply_command(ViewerCommand::SetFollowTail { enabled: true });
    state.apply_command(ViewerCommand::SetSupercell { repeats: [1, 0, 0] });
    state.apply_command(ViewerCommand::ReplaceImageSelection {
        selection: vec![
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [1, 0, 0],
            },
            main_image(0),
        ],
        frame_index: Some(0),
    });

    state.apply_command(ViewerCommand::AppendFrame {
        frame: test_structure(1.0),
    });

    assert_eq!(state.current, 1);
    assert_eq!(
        state.selected_images(1),
        vec![
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [1, 0, 0],
            },
            main_image(0),
        ]
    );
    assert_eq!(state.selected_atoms(1), vec![1, 0]);
}

#[test]
fn follow_tail_append_leaves_incompatible_frame_selection_unchanged() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    state.apply_command(ViewerCommand::SetFollowTail { enabled: true });
    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: Some(0),
    });

    state.apply_command(ViewerCommand::AppendFrame {
        frame: test_structure_with_numbers(1.0, &[8, 1]),
    });

    assert_eq!(state.current, 1);
    assert!(state.selected_atoms(1).is_empty());
    assert!(state.selected_images(1).is_empty());
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
fn supercell_commands_do_not_request_camera_reset() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    state.needs_camera_reset = false;

    state.apply_command(ViewerCommand::IncrementSupercellAxis { axis: 0 });
    assert_eq!(state.supercell.repeats, [1, 0, 0]);
    assert!(state.needs_render);
    assert!(!state.needs_camera_reset);

    state.needs_render = false;
    state.apply_command(ViewerCommand::SetSupercell { repeats: [2, 1, 0] });
    assert_eq!(state.supercell.repeats, [2, 1, 0]);
    assert!(state.needs_render);
    assert!(!state.needs_camera_reset);

    state.needs_render = false;
    state.apply_command(ViewerCommand::ResetSupercell);
    assert_eq!(state.supercell.repeats, [0, 0, 0]);
    assert!(state.needs_render);
    assert!(!state.needs_camera_reset);
}
