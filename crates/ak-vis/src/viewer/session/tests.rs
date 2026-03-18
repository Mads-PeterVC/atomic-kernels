use super::{
    AtomColorRule, BallAndStickStyle, BondFrames, BondList, BondScope, CameraState, Face,
    FaceFrames, FaceList, RenderStyle, ScalarColorMap, SelectedImageAtom, ViewerCommand,
    ViewerState, camera_view_for_frame,
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

    fn main_image(atom_index: usize) -> SelectedImageAtom {
        SelectedImageAtom {
            atom_index,
            image_offset: [0, 0, 0],
        }
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
