use super::measurement::{
    ANGLE_CUE_RADIUS_MAX, angle_measurement_cue_from_positions, render_angle_measurement_cue,
    selection_highlight_color, selection_highlight_radius,
};
use super::selection::{
    MarqueeSelectionState, image_selection_for_projected_positions, marquee_drag_exceeded,
    sync_marquee_overlay,
};
use crate::components::{FrameMeasurementCue, MainSceneCamera, MarqueeSelectionOverlay};
use crate::viewer::{DisplayAtom, SelectedImageAtom};
use bevy::ecs::system::SystemState;
use bevy::picking::prelude::Pickable;
use bevy::prelude::*;

fn display_atom(atom_index: usize, position: [f64; 3]) -> DisplayAtom {
    DisplayAtom {
        identity: SelectedImageAtom {
            atom_index,
            image_offset: [0, 0, 0],
        },
        position,
        is_main_cell: true,
    }
}

#[test]
fn selection_highlight_radius_keeps_a_minimum_extra_shell() {
    let small = selection_highlight_radius(0.25, false);
    let medium = selection_highlight_radius(0.5, false);

    assert!((small - 0.25) > 0.10);
    assert!((medium - 0.5) > 0.13);
}

#[test]
fn selection_highlight_radius_grows_sublinearly_relative_to_atom_size() {
    let small_extra = selection_highlight_radius(0.3, false) - 0.3;
    let large_extra = selection_highlight_radius(1.2, false) - 1.2;

    assert!(large_extra > small_extra);
    assert!(selection_highlight_radius(1.2, false) < 1.2 * 1.2);
}

#[test]
fn selection_highlight_color_is_more_pronounced() {
    let color = selection_highlight_color(false).to_srgba();

    assert!(color.red >= 0.99);
    assert!(color.green < 0.6);
    assert!(color.alpha >= 0.4);
}

#[test]
fn vertex_selection_highlight_is_more_emphasized() {
    assert!(selection_highlight_radius(0.6, true) > selection_highlight_radius(0.6, false));
    assert!(
        selection_highlight_color(true).to_srgba().alpha
            > selection_highlight_color(false).to_srgba().alpha
    );
}

#[test]
fn angle_measurement_cue_uses_second_selected_atom_as_vertex() {
    let cue = angle_measurement_cue_from_positions(
        &[
            display_atom(0, [1.0, 0.0, 0.0]),
            display_atom(1, [0.0, 0.0, 0.0]),
            display_atom(2, [0.0, 2.0, 0.0]),
        ],
        &[
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0],
            },
        ],
        0.0,
    )
    .unwrap();

    assert_eq!(
        cue.atoms,
        [
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0]
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0]
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0]
            },
        ]
    );
    assert_eq!(cue.vertex, Vec3::ZERO);
    assert!((cue.first_direction.length() - 1.0).abs() < 1e-6);
    assert!((cue.second_direction.length() - 1.0).abs() < 1e-6);
}

#[test]
fn angle_measurement_cue_radius_clamps_for_short_and_long_spans() {
    let short = angle_measurement_cue_from_positions(
        &[
            display_atom(0, [0.05, 0.0, 0.0]),
            display_atom(1, [0.0, 0.0, 0.0]),
            display_atom(2, [0.0, 0.05, 0.0]),
        ],
        &[
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0],
            },
        ],
        0.0,
    )
    .unwrap();
    let long = angle_measurement_cue_from_positions(
        &[
            display_atom(0, [10.0, 0.0, 0.0]),
            display_atom(1, [0.0, 0.0, 0.0]),
            display_atom(2, [0.0, 10.0, 0.0]),
        ],
        &[
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0],
            },
        ],
        0.0,
    )
    .unwrap();

    assert_eq!(short.radius, selection_highlight_radius(0.0, true) + 0.14);
    assert_eq!(long.radius, ANGLE_CUE_RADIUS_MAX);
}

#[test]
fn angle_measurement_cue_returns_none_for_degenerate_geometry() {
    assert!(
        angle_measurement_cue_from_positions(
            &[
                display_atom(0, [1.0, 0.0, 0.0]),
                display_atom(1, [1.0, 0.0, 0.0]),
                display_atom(2, [0.0, 1.0, 0.0]),
            ],
            &[
                SelectedImageAtom {
                    atom_index: 0,
                    image_offset: [0, 0, 0]
                },
                SelectedImageAtom {
                    atom_index: 1,
                    image_offset: [0, 0, 0]
                },
                SelectedImageAtom {
                    atom_index: 2,
                    image_offset: [0, 0, 0]
                },
            ],
            0.0,
        )
        .is_none()
    );
    assert!(
        angle_measurement_cue_from_positions(
            &[
                display_atom(0, [1.0, 0.0, 0.0]),
                display_atom(1, [0.0, 0.0, 0.0]),
                display_atom(2, [2.0, 0.0, 0.0]),
            ],
            &[
                SelectedImageAtom {
                    atom_index: 0,
                    image_offset: [0, 0, 0]
                },
                SelectedImageAtom {
                    atom_index: 1,
                    image_offset: [0, 0, 0]
                },
                SelectedImageAtom {
                    atom_index: 2,
                    image_offset: [0, 0, 0]
                },
            ],
            0.0,
        )
        .is_none()
    );
}

#[test]
fn angle_measurement_cue_radius_clears_emphasized_vertex_shell() {
    let vertex_atom_radius = 0.62;
    let cue = angle_measurement_cue_from_positions(
        &[
            display_atom(0, [1.0, 0.0, 0.0]),
            display_atom(1, [0.0, 0.0, 0.0]),
            display_atom(2, [0.0, 1.0, 0.0]),
        ],
        &[
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0],
            },
        ],
        vertex_atom_radius,
    )
    .unwrap();

    assert!(cue.radius > selection_highlight_radius(vertex_atom_radius, true));
}

#[test]
fn render_angle_measurement_cue_spawns_non_pickable_entities() {
    let cue = angle_measurement_cue_from_positions(
        &[
            display_atom(0, [1.0, 0.0, 0.0]),
            display_atom(1, [0.0, 0.0, 0.0]),
            display_atom(2, [0.0, 1.0, 0.0]),
        ],
        &[
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0],
            },
            SelectedImageAtom {
                atom_index: 2,
                image_offset: [0, 0, 0],
            },
        ],
        0.0,
    )
    .unwrap();
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();

    let mut system_state: SystemState<(
        Commands,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )> = SystemState::new(app.world_mut());

    let (mut commands, mut meshes, mut materials) = system_state.get_mut(app.world_mut());
    render_angle_measurement_cue(&cue, &mut commands, &mut materials, &mut meshes);
    system_state.apply(app.world_mut());

    let entities: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<FrameMeasurementCue>>()
        .iter(app.world())
        .collect();
    assert!(entities.len() >= 3);
    assert!(
        entities
            .iter()
            .all(|entity| app.world().get::<Pickable>(*entity) == Some(&Pickable::IGNORE))
    );
}

#[test]
fn marquee_threshold_requires_real_drag_distance() {
    assert!(!marquee_drag_exceeded(
        Vec2::new(10.0, 10.0),
        Vec2::new(13.0, 14.0)
    ));
    assert!(marquee_drag_exceeded(
        Vec2::new(10.0, 10.0),
        Vec2::new(20.0, 10.0)
    ));
}

#[test]
fn projected_selection_mask_is_depth_agnostic() {
    let rect = Rect::from_corners(Vec2::new(10.0, 10.0), Vec2::new(40.0, 40.0));
    let mask = image_selection_for_projected_positions(
        [
            (
                SelectedImageAtom {
                    atom_index: 0,
                    image_offset: [0, 0, 0],
                },
                Vec2::new(12.0, 12.0),
            ),
            (
                SelectedImageAtom {
                    atom_index: 1,
                    image_offset: [0, 0, 0],
                },
                Vec2::new(12.0, 12.0),
            ),
            (
                SelectedImageAtom {
                    atom_index: 2,
                    image_offset: [0, 0, 0],
                },
                Vec2::new(60.0, 60.0),
            ),
            (
                SelectedImageAtom {
                    atom_index: 3,
                    image_offset: [0, 0, 0],
                },
                Vec2::new(20.0, 25.0),
            ),
        ],
        rect,
    );

    assert_eq!(
        mask,
        vec![
            SelectedImageAtom {
                atom_index: 0,
                image_offset: [0, 0, 0]
            },
            SelectedImageAtom {
                atom_index: 1,
                image_offset: [0, 0, 0]
            },
            SelectedImageAtom {
                atom_index: 3,
                image_offset: [0, 0, 0]
            },
        ]
    );
}

#[test]
fn sync_marquee_overlay_updates_visibility_and_bounds() {
    let mut world = World::new();
    let mut marquee = MarqueeSelectionState::default();
    marquee.begin(Vec2::new(10.0, 15.0));
    marquee.update(Vec2::new(40.0, 55.0));
    world.insert_resource(marquee);
    world.spawn((Camera::default(), MainSceneCamera));
    world.spawn((
        Node {
            display: Display::None,
            ..default()
        },
        Visibility::Hidden,
        MarqueeSelectionOverlay,
    ));

    let mut system_state: SystemState<(
        Commands,
        Res<MarqueeSelectionState>,
        Query<Entity, With<MainSceneCamera>>,
        Query<
            (Entity, &mut Node, &mut Visibility, Option<&UiTargetCamera>),
            With<MarqueeSelectionOverlay>,
        >,
    )> = SystemState::new(&mut world);

    let (commands, marquee, main_camera, overlays) = system_state.get_mut(&mut world);
    sync_marquee_overlay(commands, marquee, main_camera, overlays);
    system_state.apply(&mut world);

    let main_camera_entity = world
        .query_filtered::<Entity, With<MainSceneCamera>>()
        .single(&world)
        .expect("main camera should exist");
    let (node, visibility, target_camera) = world
        .query::<(&Node, &Visibility, &UiTargetCamera)>()
        .single(&world)
        .expect("overlay should exist");
    assert_eq!(node.display, Display::Flex);
    assert_eq!(node.left, px(10.0));
    assert_eq!(node.top, px(15.0));
    assert_eq!(node.width, px(30.0));
    assert_eq!(node.height, px(40.0));
    assert_eq!(*visibility, Visibility::Visible);
    assert_eq!(target_camera.entity(), main_camera_entity);
}
