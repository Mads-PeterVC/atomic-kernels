use super::helpers::*;
use crate::viewer::session::{SelectedImageAtom, ViewerCommand, ViewerState};
use ak_core::Trajectory;
#[test]
fn selection_commands_update_current_frame_state() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: None,
    });
    state.apply_command(ViewerCommand::AddSelection {
        selection: vec![false, true],
        frame_index: None,
    });
    state.apply_command(ViewerCommand::RemoveSelection {
        selection: vec![true, false],
        frame_index: None,
    });

    assert_eq!(state.selected_atoms(0), vec![1]);
}

#[test]
fn selection_state_is_frame_scoped() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![test_structure(0.0), test_structure(1.0)]),
        0,
    );

    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: Some(0),
    });
    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![false, true],
        frame_index: Some(1),
    });

    assert_eq!(state.selected_atoms(0), vec![0]);
    assert_eq!(state.selected_atoms(1), vec![1]);
}

#[test]
fn set_current_frame_copies_selection_to_compatible_frame() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![test_structure(0.0), test_structure(1.0)]),
        0,
    );

    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![false, true],
        frame_index: Some(0),
    });
    state.apply_command(ViewerCommand::SetCurrentFrame { index: 1 });

    assert_eq!(state.current, 1);
    assert_eq!(state.selected_atoms(1), vec![1]);
}

#[test]
fn set_current_frame_copies_selection_order_to_compatible_frame() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![test_structure4(), test_structure4()]),
        0,
    );

    assert!(state.toggle_atom_selection(main_image(2)));
    assert!(state.toggle_atom_selection(main_image(0)));
    assert!(state.toggle_atom_selection(main_image(3)));

    state.apply_command(ViewerCommand::SetCurrentFrame { index: 1 });

    assert_eq!(state.selected_atoms(1), vec![2, 0, 3]);
}

#[test]
fn set_current_frame_overwrites_existing_selection_on_compatible_frame() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![test_structure(0.0), test_structure(1.0)]),
        0,
    );

    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: Some(0),
    });
    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![false, true],
        frame_index: Some(1),
    });

    state.apply_command(ViewerCommand::SetCurrentFrame { index: 1 });

    assert_eq!(state.selected_atoms(1), vec![0]);
}

#[test]
fn set_current_frame_does_not_copy_selection_to_incompatible_frame() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![
            test_structure_with_numbers(0.0, &[1, 8]),
            test_structure_with_numbers(1.0, &[8, 1]),
        ]),
        0,
    );

    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: Some(0),
    });
    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![false, true],
        frame_index: Some(1),
    });

    state.apply_command(ViewerCommand::SetCurrentFrame { index: 1 });

    assert_eq!(state.selected_atoms(1), vec![1]);
}

#[test]
fn step_frame_copies_image_selection_to_compatible_frame() {
    let mut state = ViewerState::new(
        Trajectory::new(vec![test_structure(0.0), test_structure(1.0)]),
        0,
    );
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

    assert!(state.step_frame(1));

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
fn click_selection_preserves_toggle_order() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure4()]), 0);

    assert!(state.toggle_atom_selection(main_image(2)));
    assert!(state.toggle_atom_selection(main_image(0)));
    assert!(state.toggle_atom_selection(main_image(3)));

    assert_eq!(state.selected_atoms(0), vec![2, 0, 3]);
}

#[test]
fn toggling_atom_off_removes_it_from_selection_order() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure4()]), 0);

    assert!(state.toggle_atom_selection(main_image(2)));
    assert!(state.toggle_atom_selection(main_image(0)));
    assert!(state.toggle_atom_selection(main_image(2)));

    assert_eq!(state.selected_atoms(0), vec![0]);
}

#[test]
fn load_trajectory_resets_selection_state() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);
    state.apply_command(ViewerCommand::ReplaceSelection {
        selection: vec![true, false],
        frame_index: None,
    });

    state.apply_command(ViewerCommand::LoadTrajectory {
        frames: vec![test_structure4()],
        initial_frame: 0,
    });

    assert!(state.selected_atoms(0).is_empty());
    assert_eq!(state.current_selection().len(), 4);
}
