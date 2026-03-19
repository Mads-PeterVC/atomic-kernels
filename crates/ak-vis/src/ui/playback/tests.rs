use super::{
    PlaybackState, set_camera_input_enabled, sync_playback_slider_value, sync_playback_state,
};
use crate::components::PlaybackScrubberButton;
use crate::viewer::ViewerState;
use ak_core::{Structure, Trajectory};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{CoreSliderDragState, SliderRange, SliderValue};
use bevy_panorbit_camera::PanOrbitCamera;

fn structure(offset: f64) -> Structure {
    Structure::new(
        vec![[offset, 0.0, 0.0]],
        vec![1],
        [[5.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 5.0]],
        [false; 3],
    )
}

#[test]
fn sync_playback_state_mirrors_viewer_state() {
    let mut app = App::new();
    app.insert_resource(ViewerState::new(
        Trajectory::new(vec![structure(0.0), structure(1.0), structure(2.0)]),
        1,
    ));
    app.insert_resource(PlaybackState::default());
    app.add_systems(Update, sync_playback_state);

    app.update();

    let playback = app.world().resource::<PlaybackState>();
    assert_eq!(playback.current_frame, 1);
    assert_eq!(playback.total_frames, 3);
    assert!(!playback.follow_tail);
}

#[test]
fn sync_playback_slider_value_sets_frame_from_widget_value() {
    let mut app = App::new();
    app.insert_resource(ViewerState::new(
        Trajectory::new(vec![
            structure(0.0),
            structure(1.0),
            structure(2.0),
            structure(3.0),
        ]),
        0,
    ));
    app.insert_resource(PlaybackState::default());
    app.world_mut().spawn((
        PlaybackScrubberButton,
        SliderValue(3.0),
        SliderRange::new(0.0, 3.0),
        Hovered::default(),
        CoreSliderDragState::default(),
    ));
    app.add_systems(Update, sync_playback_slider_value);

    app.update();

    assert_eq!(app.world().resource::<ViewerState>().current, 3);
}

#[test]
fn set_camera_input_enabled_disables_camera_while_scrubbing() {
    let mut app = App::new();
    let mut drag_state = CoreSliderDragState::default();
    drag_state.dragging = true;
    app.world_mut().spawn((PlaybackScrubberButton, drag_state));
    let camera = app.world_mut().spawn(PanOrbitCamera::default()).id();
    app.add_systems(Update, set_camera_input_enabled);

    app.update();

    assert!(!app.world().get::<PanOrbitCamera>(camera).unwrap().enabled);
}
