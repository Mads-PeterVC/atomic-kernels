use ak_core::{Structure, Trajectory};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use std::sync::{Arc, mpsc};
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
use crate::viewer::runtime::{asset_root, configure_shared_app};
use crate::viewer::session::{ViewerCommand, ViewerReadiness, ViewerSessionHandle};

#[derive(Resource, Clone)]
struct ViewerLifecycle {
    readiness: Arc<ViewerReadiness>,
    ready_signaled: bool,
}

impl Drop for ViewerLifecycle {
    fn drop(&mut self) {
        self.readiness.mark_closed();
    }
}

fn build_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    readiness: Arc<ViewerReadiness>,
) -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .build()
            .disable::<bevy::audio::AudioPlugin>()
            .set(AssetPlugin {
                file_path: asset_root(),
                ..default()
            }),
        MeshPickingPlugin,
        PanOrbitCameraPlugin,
    ));
    configure_shared_app(&mut app, trajectory, config, receiver);
    app.insert_resource(ViewerLifecycle {
        readiness,
        ready_signaled: false,
    })
    .add_systems(Startup, setup_orientation_widget)
    .add_systems(
        Update,
        (
            toggle_view,
            keyboard_controls,
            screenshot_on_spacebar,
            screenshot_saving,
            navigate_frames,
            sync_orientation_widget,
            sync_orientation_widget_letter_strokes,
            update_orientation_widget_viewport,
            signal_viewer_ready,
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
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    readiness: Arc<ViewerReadiness>,
) {
    let mut app = build_app(trajectory, config, receiver, readiness);
    app.run();
}

fn signal_viewer_ready(
    mut lifecycle: ResMut<'_, ViewerLifecycle>,
    windows: Query<'_, '_, Entity, With<Window>>,
) {
    if lifecycle.ready_signaled || windows.is_empty() {
        return;
    }

    lifecycle.ready_signaled = true;
    lifecycle.readiness.mark_ready();
}

pub fn run_prepared(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: mpsc::Receiver<ViewerCommand>,
    readiness: Arc<ViewerReadiness>,
) {
    run_app(trajectory, config, Some(receiver), readiness);
}

pub fn launch(trajectory: Trajectory, config: ViewerConfig) -> ViewerSessionHandle {
    let (sender, receiver) = mpsc::channel();
    let readiness = Arc::new(ViewerReadiness::new());
    let launch_readiness = readiness.clone();

    thread::Builder::new()
        .name("ak-viewer-session".to_string())
        .spawn(move || {
            run_app(trajectory, config, Some(receiver), launch_readiness);
        })
        .expect("failed to launch viewer session thread");

    ViewerSessionHandle::with_readiness(sender, readiness)
}

pub fn run_with_session<F>(trajectory: Trajectory, config: ViewerConfig, driver: F)
where
    F: FnOnce(ViewerSessionHandle) + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let readiness = Arc::new(ViewerReadiness::new());
    let handle = ViewerSessionHandle::with_readiness(sender, readiness.clone());

    thread::Builder::new()
        .name("ak-viewer-session-driver".to_string())
        .spawn(move || {
            driver(handle);
        })
        .expect("failed to launch viewer session driver thread");

    run_app(trajectory, config, Some(receiver), readiness);
}

pub fn run(trajectory: Trajectory, config: ViewerConfig) {
    run_app(trajectory, config, None, Arc::new(ViewerReadiness::new()));
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
