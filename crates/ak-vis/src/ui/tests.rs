use super::{
    InspectorState, MeasurementStatus, SelectedAtomSummary, derive_inspector_state,
    sync_inspector_camera, sync_inspector_text, toggle_hints_visibility,
};
use super::shortcuts::shortcut_hints;
use crate::components::{
    InspectorHintsContainer, InspectorHintsToggle, InspectorMeasurementBody,
    InspectorMeasurementSection, InspectorPanelRoot, InspectorSelectionBody,
    InspectorSelectionSection, MainSceneCamera,
};
use crate::viewer::ViewerState;
use ak_core::{Structure, Trajectory};
use bevy::prelude::*;

fn structure(points: &[[f64; 3]], numbers: &[i32]) -> Structure {
    Structure::new(
        points.to_vec(),
        numbers.to_vec(),
        [[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, 10.0]],
        [false, false, false],
    )
}

#[test]
fn derive_inspector_state_reports_distance_for_two_selected_atoms() {
    let traj = Trajectory::new(vec![structure(&[[0.0, 0.0, 0.0], [0.0, 3.0, 4.0]], &[1, 8])]);
    let mut viewer = ViewerState::new(traj, 0);
    viewer.selection.replace(0, vec![true, true]);

    let inspector = derive_inspector_state(&viewer);

    assert_eq!(inspector.selected_atoms.len(), 2);
    assert_eq!(inspector.selected_atoms[0].symbol, "H");
    assert!(matches!(
        inspector.measurement,
        MeasurementStatus::Distance { angstrom, .. } if (angstrom - 5.0).abs() < 1e-9
    ));
}

#[test]
fn derive_inspector_state_reports_angle_for_three_selected_atoms() {
    let traj = Trajectory::new(vec![structure(
        &[[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        &[1, 6, 8],
    )]);
    let mut viewer = ViewerState::new(traj, 0);
    viewer.selection.replace(0, vec![true, true, true]);

    let inspector = derive_inspector_state(&viewer);

    assert!(matches!(
        inspector.measurement,
        MeasurementStatus::Angle { degrees, .. } if (degrees - 90.0).abs() < 1e-9
    ));
}

#[test]
fn derive_inspector_state_reports_selection_limits() {
    let traj = Trajectory::new(vec![structure(
        &[
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
        ],
        &[1, 1, 1, 1],
    )]);
    let mut viewer = ViewerState::new(traj, 0);

    let empty = derive_inspector_state(&viewer);
    assert_eq!(empty.measurement, MeasurementStatus::Empty);

    viewer.selection.replace(0, vec![true, false, false, false]);
    let one = derive_inspector_state(&viewer);
    assert_eq!(one.measurement, MeasurementStatus::NeedOneMoreAtom);

    viewer.selection.replace(0, vec![true, true, true, true]);
    let many = derive_inspector_state(&viewer);
    assert_eq!(
        many.measurement,
        MeasurementStatus::UnsupportedCount { count: 4 }
    );
}

#[test]
fn derive_inspector_state_uses_current_frame_coordinates() {
    let traj = Trajectory::new(vec![
        structure(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], &[1, 1]),
        structure(&[[0.0, 0.0, 0.0], [2.5, 0.0, 0.0]], &[1, 1]),
    ]);
    let mut viewer = ViewerState::new(traj, 0);
    viewer.selection.replace(0, vec![true, true]);
    viewer.selection.replace(1, vec![true, true]);
    viewer.current = 1;

    let inspector = derive_inspector_state(&viewer);

    assert!(matches!(
        inspector.measurement,
        MeasurementStatus::Distance { angstrom, .. } if (angstrom - 2.5).abs() < 1e-9
    ));
}

#[test]
fn shortcut_hints_cover_current_viewer_controls() {
    let labels: Vec<&str> = shortcut_hints().iter().map(|hint| hint.label).collect();

    assert!(labels.contains(&"replace selection"));
    assert!(labels.contains(&"toggle atom"));
    assert!(labels.contains(&"marquee replace"));
    assert!(labels.contains(&"step frames"));
    assert!(labels.contains(&"zoom"));
    assert!(labels.contains(&"orbit camera"));
    assert!(labels.contains(&"snap to axes"));
    assert!(labels.contains(&"save screenshot"));
    assert!(labels.contains(&"toggle UI"));
    assert!(labels.contains(&"toggle keybindings"));
}

#[test]
fn sync_inspector_text_updates_all_sections() {
    let mut app = App::new();
    app.insert_resource(InspectorState {
        selected_atoms: vec![
            SelectedAtomSummary {
                index: 0,
                symbol: "H".to_string(),
            },
            SelectedAtomSummary {
                index: 2,
                symbol: "O".to_string(),
            },
        ],
        measurement: MeasurementStatus::Distance {
            atoms: [0, 2],
            angstrom: 1.2345,
        },
        hints_expanded: true,
    });
    let selection = app
        .world_mut()
        .spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorSelectionSection,
        ))
        .with_child((Text::new(""), InspectorSelectionBody))
        .id();
    let measurement = app
        .world_mut()
        .spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorMeasurementSection,
        ))
        .with_child((Text::new(""), InspectorMeasurementBody))
        .id();
    let hints_toggle = app
        .world_mut()
        .spawn((Text::new(""), InspectorHintsToggle))
        .id();
    app.add_systems(Update, sync_inspector_text);

    app.update();

    assert!(
        app.world()
            .entity(selection)
            .get::<Children>()
            .and_then(|children| children.first().copied())
            .and_then(|child| app.world().get::<Text>(child))
            .unwrap()
            .0
            .contains("#0 H, #2 O")
    );
    assert!(
        app.world()
            .entity(measurement)
            .get::<Children>()
            .and_then(|children| children.first().copied())
            .and_then(|child| app.world().get::<Text>(child))
            .unwrap()
            .0
            .contains("1.234")
    );
    assert_eq!(
        app.world().get::<Text>(hints_toggle).unwrap().0,
        ")"
    );
}

#[test]
fn sync_inspector_text_hides_selection_section_when_empty() {
    let mut app = App::new();
    app.insert_resource(InspectorState::default());
    let section = app
        .world_mut()
        .spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorSelectionSection,
        ))
        .id();
    app.world_mut()
        .entity_mut(section)
        .with_child((Text::new(""), InspectorSelectionBody));
    app.world_mut()
        .spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorMeasurementSection,
        ))
        .with_child((Text::new(""), InspectorMeasurementBody));
    app.world_mut().spawn((Text::new(""), InspectorHintsToggle));
    app.add_systems(Update, sync_inspector_text);

    app.update();

    assert_eq!(app.world().get::<Node>(section).unwrap().display, Display::None);
}

#[test]
fn sync_inspector_text_hides_measurement_section_without_result() {
    let mut app = App::new();
    app.insert_resource(InspectorState::default());
    app.world_mut().spawn((
        Node {
            display: Display::Flex,
            ..default()
        },
        InspectorSelectionSection,
    ));
    let section = app
        .world_mut()
        .spawn((
            Node {
                display: Display::Flex,
                ..default()
            },
            InspectorMeasurementSection,
        ))
        .id();
    app.world_mut()
        .entity_mut(section)
        .with_child((Text::new(""), InspectorMeasurementBody));
    app.world_mut().spawn((Text::new(""), InspectorHintsToggle));
    app.add_systems(Update, sync_inspector_text);

    app.update();

    assert_eq!(app.world().get::<Node>(section).unwrap().display, Display::None);
}

#[test]
fn sync_inspector_camera_targets_main_scene_camera() {
    let mut app = App::new();
    let main_camera = app.world_mut().spawn((Camera::default(), MainSceneCamera)).id();
    let inspector = app.world_mut().spawn(InspectorPanelRoot).id();
    app.add_systems(Update, sync_inspector_camera);

    app.update();

    assert_eq!(
        app.world().get::<UiTargetCamera>(inspector).unwrap().0,
        main_camera
    );
}

#[test]
fn toggle_hints_visibility_flips_body_and_state_on_h_press() {
    let mut app = App::new();
    app.insert_resource(InspectorState::default());
    let entity = app
        .world_mut()
        .spawn((
            InspectorHintsContainer,
            Node {
                display: Display::None,
                ..default()
            },
        ))
        .id();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyH);
    app.insert_resource(keys);
    app.add_systems(Update, toggle_hints_visibility);

    app.update();

    assert!(app.world().resource::<InspectorState>().hints_expanded);
    assert_eq!(
        app.world().get::<Node>(entity).unwrap().display,
        Display::Flex
    );
}
