use ak_core::{Structure, Trajectory};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use std::sync::{Mutex, mpsc};
use std::thread;

use crate::ui::setup_ui;
use crate::viewer::ViewerConfig;
use crate::viewer::controls::{
    keyboard_controls, navigate_frames, screenshot_on_spacebar, screenshot_saving,
    toggle_ui_visibility, toggle_view,
};
use crate::viewer::orientation_widget::{
    setup_orientation_widget, sync_orientation_widget, sync_orientation_widget_letter_strokes,
    update_orientation_widget_viewport,
};
use crate::viewer::session::{CameraState, ViewerSessionHandle, ViewerState};
use crate::viewer::systems::{
    advance_camera_motion, apply_camera_state, apply_viewer_commands, render_current_frame,
    rerender_if_dirty, setup_camera, setup_camera_light, setup_lighting, update_camera_light,
};

#[derive(Resource)]
pub struct CommandReceiver(pub Option<Mutex<mpsc::Receiver<crate::viewer::ViewerCommand>>>);

fn asset_root() -> String {
    format!("{}/assets", env!("CARGO_MANIFEST_DIR"))
}

fn build_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<crate::viewer::ViewerCommand>>,
) -> App {
    let viewer_state = ViewerState::new(trajectory, config.initial_frame);
    let camera_state = CameraState::new(&viewer_state);
    let mut app = App::new();
    app.insert_resource(ClearColor(config.color.background))
        .insert_resource(viewer_state)
        .insert_resource(camera_state)
        .insert_resource(config)
        .insert_resource(CommandReceiver(receiver.map(Mutex::new)))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: asset_root(),
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(
            Startup,
            (
                setup_lighting,
                setup_camera,
                render_current_frame,
                setup_camera_light,
                setup_orientation_widget,
            ),
        )
        .add_systems(
            Update,
            (
                apply_viewer_commands,
                advance_camera_motion,
                apply_camera_state,
                toggle_view,
                keyboard_controls,
                update_camera_light,
                screenshot_on_spacebar,
                screenshot_saving,
                navigate_frames,
                rerender_if_dirty,
                sync_orientation_widget,
                sync_orientation_widget_letter_strokes,
                update_orientation_widget_viewport,
            ),
        );

    if app.world().resource::<ViewerConfig>().render.show_ui {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, toggle_ui_visibility);
    }

    app
}

fn run_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<crate::viewer::ViewerCommand>>,
) {
    let mut app = build_app(trajectory, config, receiver);
    app.run();
}

pub fn run_prepared(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: mpsc::Receiver<crate::viewer::ViewerCommand>,
) {
    run_app(trajectory, config, Some(receiver));
}

pub fn launch(trajectory: Trajectory, config: ViewerConfig) -> ViewerSessionHandle {
    let (sender, receiver) = mpsc::channel();

    thread::Builder::new()
        .name("ak-viewer-session".to_string())
        .spawn(move || {
            run_app(trajectory, config, Some(receiver));
        })
        .expect("failed to launch viewer session thread");

    ViewerSessionHandle::new(sender)
}

pub fn run_with_session<F>(trajectory: Trajectory, config: ViewerConfig, driver: F)
where
    F: FnOnce(ViewerSessionHandle) + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let handle = ViewerSessionHandle::new(sender);

    thread::Builder::new()
        .name("ak-viewer-session-driver".to_string())
        .spawn(move || {
            driver(handle);
        })
        .expect("failed to launch viewer session driver thread");

    run_app(trajectory, config, Some(receiver));
}

pub fn run(trajectory: Trajectory, config: ViewerConfig) {
    run_app(trajectory, config, None);
}

pub fn run_default(trajectory: Trajectory) {
    let config = ViewerConfig::default();
    run(trajectory, config)
}

pub fn run_structure(structure: Structure, config: ViewerConfig) {
    let trajectory = Trajectory::new(vec![structure]);
    run(trajectory, config)
}

pub fn run_structure_default(structure: Structure) {
    let trajectory = Trajectory::new(vec![structure]);
    run_default(trajectory);
}
