use super::helpers::*;
use crate::viewer::session::{
    BondFrames, BondList, Face, FaceFrames, FaceList, ViewerCommand, ViewerState,
};
use ak_core::Trajectory;
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
fn bond_frames_add_remove_and_clear_bonds() {
    let mut frames = BondFrames::new(1);
    frames.set(0, BondList::new([(0, 1)]));
    frames.add(0, BondList::new([(1, 0), (0, 2)]));

    assert_eq!(
        frames.get(0).unwrap().iter().copied().collect::<Vec<_>>(),
        vec![(0, 1), (0, 2)]
    );

    frames.remove(0, &BondList::new([(0, 1)]));
    assert_eq!(
        frames.get(0).unwrap().iter().copied().collect::<Vec<_>>(),
        vec![(0, 2)]
    );

    frames.clear(0);
    assert!(frames.get(0).is_none());
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
fn face_frames_add_remove_and_clear_faces() {
    let mut frames = FaceFrames::new(1);
    frames.set(
        0,
        FaceList::new([Face::new([0, 1, 2], [0.2, 0.4, 0.6, 0.3]).unwrap()]),
    );
    frames.add(
        0,
        FaceList::new([
            Face::new([2, 1, 0], [0.9, 0.2, 0.1, 0.3]).unwrap(),
            Face::new([0, 2, 3], [0.5, 0.5, 0.5, 0.3]).unwrap(),
        ]),
    );

    assert_eq!(frames.get(0).unwrap().len(), 2);

    frames.remove(
        0,
        &FaceList::new([Face::new([2, 1, 0], [1.0, 1.0, 1.0, 1.0]).unwrap()]),
    );
    assert_eq!(frames.get(0).unwrap().len(), 1);
    assert_eq!(
        frames
            .get(0)
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .atoms
            .as_slice(),
        &[0, 2, 3]
    );

    frames.clear(0);
    assert!(frames.get(0).is_none());
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
fn bond_mutation_commands_update_frame_bonds() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure4()]), 0);

    state.apply_command(ViewerCommand::SetBonds {
        bonds: BondList::new([(0, 1)]),
        frame_index: None,
    });
    state.apply_command(ViewerCommand::AddBonds {
        bonds: BondList::new([(1, 0), (0, 2)]),
        frame_index: None,
    });
    state.apply_command(ViewerCommand::RemoveBonds {
        bonds: BondList::new([(0, 1)]),
        frame_index: None,
    });

    assert_eq!(
        state
            .bonds
            .get(0)
            .unwrap()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![(0, 2)]
    );

    state.apply_command(ViewerCommand::ClearBonds { frame_index: None });
    assert!(state.bonds.get(0).is_none());
}

#[test]
fn face_mutation_commands_update_frame_faces() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure4()]), 0);

    state.apply_command(ViewerCommand::SetFaces {
        faces: FaceList::new([Face::new([0, 1, 2], [0.1, 0.2, 0.3, 0.4]).unwrap()]),
        frame_index: None,
    });
    state.apply_command(ViewerCommand::AddFaces {
        faces: FaceList::new([
            Face::new([2, 1, 0], [0.8, 0.8, 0.8, 0.4]).unwrap(),
            Face::new([0, 2, 3], [0.2, 0.4, 0.6, 0.3]).unwrap(),
        ]),
        frame_index: None,
    });
    state.apply_command(ViewerCommand::RemoveFaces {
        faces: FaceList::new([Face::new([2, 1, 0], [1.0, 1.0, 1.0, 1.0]).unwrap()]),
        frame_index: None,
    });

    let faces = state.faces.get(0).unwrap();
    assert_eq!(faces.len(), 1);
    assert_eq!(faces.iter().next().unwrap().atoms.as_slice(), &[0, 2, 3]);

    state.apply_command(ViewerCommand::ClearFaces { frame_index: None });
    assert!(state.faces.get(0).is_none());
}
