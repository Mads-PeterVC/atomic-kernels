use ak_core::{Structure, Trajectory};
use bevy::{input_focus::InputDispatchPlugin, prelude::*, ui_widgets::UiWidgetsPlugins};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCameraSystemSet};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::ui::{
    handle_playback_buttons, set_camera_input_enabled, setup_viewer_ui, sync_inspector_camera,
    sync_inspector_state, sync_inspector_text, sync_playback_camera, sync_playback_slider_value,
    sync_playback_state, sync_playback_text, sync_playback_visibility, toggle_hints_visibility,
};
use crate::viewer::ViewerConfig;
use crate::viewer::controls::{
    keyboard_controls, navigate_frames, screenshot_on_spacebar, screenshot_saving,
    toggle_ui_visibility, toggle_view,
};
use crate::viewer::orientation_widget::{
    setup_orientation_widget, sync_orientation_widget, sync_orientation_widget_letter_strokes,
    update_orientation_widget_viewport,
};
use crate::viewer::runtime::{configure_shared_app, register_viewer_fonts};
use crate::viewer::session::{ViewerCommand, ViewerReadiness, ViewerSessionHandle};
use crate::viewer::systems::{
    MarqueeSelectionState, handle_atom_clicks, handle_marquee_selection, setup_marquee_overlay,
    sync_marquee_overlay,
};

#[derive(Clone, Debug, Default)]
pub struct ViewerAppOptions {
    pub canvas_selector: Option<String>,
    pub fit_canvas_to_parent: bool,
}

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

pub fn build_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    readiness: Arc<ViewerReadiness>,
    snapshot: Arc<Mutex<crate::viewer::session::ViewerSnapshot>>,
) -> App {
    build_app_with_options(
        trajectory,
        config,
        receiver,
        readiness,
        snapshot,
        ViewerAppOptions::default(),
    )
}

pub fn build_app_with_options(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    readiness: Arc<ViewerReadiness>,
    snapshot: Arc<Mutex<crate::viewer::session::ViewerSnapshot>>,
    options: ViewerAppOptions,
) -> App {
    let mut app = App::new();
    let mut plugins = DefaultPlugins.build().disable::<bevy::audio::AudioPlugin>();

    if let Some(primary_window) = primary_window_override(&config, &options) {
        plugins = plugins.set(WindowPlugin {
            primary_window: Some(primary_window),
            ..default()
        });
    }

    app.add_plugins((
        plugins,
        MeshPickingPlugin,
        PanOrbitCameraPlugin,
        UiWidgetsPlugins,
        InputDispatchPlugin,
    ));
    register_viewer_fonts(&mut app);
    configure_shared_app(&mut app, trajectory, config, receiver, snapshot);
    app.insert_resource(MarqueeSelectionState::default());
    app.insert_resource(ViewerLifecycle {
        readiness,
        ready_signaled: false,
    })
    .add_systems(Startup, (setup_orientation_widget, setup_marquee_overlay))
    .add_systems(
        Update,
        (
            (
                handle_marquee_selection,
                handle_atom_clicks,
                sync_marquee_overlay,
            )
                .chain(),
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
        app.add_systems(Startup, setup_viewer_ui);
        app.add_systems(
            Update,
            (
                toggle_ui_visibility,
                toggle_hints_visibility,
                handle_playback_buttons,
                sync_playback_slider_value,
            ),
        );
        app.add_systems(
            PostUpdate,
            set_camera_input_enabled.before(PanOrbitCameraSystemSet),
        );
        app.add_systems(
            PostUpdate,
            (
                sync_inspector_camera,
                sync_inspector_state,
                sync_inspector_text,
                sync_playback_camera,
                sync_playback_state,
                sync_playback_visibility,
                sync_playback_text,
            )
                .chain(),
        );
    }

    app
}

fn primary_window_override(config: &ViewerConfig, options: &ViewerAppOptions) -> Option<Window> {
    let needs_size_override = config.window_width.is_some() || config.window_height.is_some();
    let needs_canvas_override = options.canvas_selector.is_some() || options.fit_canvas_to_parent;

    if !needs_size_override && !needs_canvas_override {
        return None;
    }

    let mut window = Window::default();

    if needs_size_override {
        let width = config.window_width.unwrap_or(750);
        let height = config.window_height.unwrap_or(750);
        window.resolution = (width, height).into();
    }

    window.canvas.clone_from(&options.canvas_selector);
    window.fit_canvas_to_parent = options.fit_canvas_to_parent;

    Some(window)
}

fn run_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    readiness: Arc<ViewerReadiness>,
    snapshot: Arc<Mutex<crate::viewer::session::ViewerSnapshot>>,
) {
    let mut app = build_app(trajectory, config, receiver, readiness, snapshot);
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
    snapshot: Arc<Mutex<crate::viewer::session::ViewerSnapshot>>,
) {
    run_app(trajectory, config, Some(receiver), readiness, snapshot);
}

pub fn launch(trajectory: Trajectory, config: ViewerConfig) -> ViewerSessionHandle {
    let (sender, receiver) = mpsc::channel();
    let handle = ViewerSessionHandle::new(sender);
    let launch_readiness = handle.readiness().clone();
    let launch_snapshot = handle.snapshot().clone();

    thread::Builder::new()
        .name("ak-viewer-session".to_string())
        .spawn(move || {
            run_app(
                trajectory,
                config,
                Some(receiver),
                launch_readiness,
                launch_snapshot,
            );
        })
        .expect("failed to launch viewer session thread");

    handle
}

pub fn run_with_session<F>(trajectory: Trajectory, config: ViewerConfig, driver: F)
where
    F: FnOnce(ViewerSessionHandle) + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let handle = ViewerSessionHandle::new(sender);
    let readiness = handle.readiness().clone();
    let snapshot = handle.snapshot().clone();

    thread::Builder::new()
        .name("ak-viewer-session-driver".to_string())
        .spawn(move || {
            driver(handle);
        })
        .expect("failed to launch viewer session driver thread");

    run_app(trajectory, config, Some(receiver), readiness, snapshot);
}

pub fn run(trajectory: Trajectory, config: ViewerConfig) {
    run_app(
        trajectory,
        config,
        None,
        Arc::new(ViewerReadiness::new()),
        Arc::new(Mutex::new(crate::viewer::session::ViewerSnapshot::default())),
    );
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

#[cfg(test)]
mod tests {
    use super::{ViewerAppOptions, primary_window_override};
    use crate::viewer::ViewerConfig;

    #[test]
    fn default_app_options_do_not_override_primary_window() {
        let config = ViewerConfig::default();
        let options = ViewerAppOptions::default();

        assert!(primary_window_override(&config, &options).is_none());
    }

    #[test]
    fn canvas_selector_overrides_primary_window_canvas() {
        let config = ViewerConfig::default();
        let options = ViewerAppOptions {
            canvas_selector: Some("#viewer-canvas".to_string()),
            fit_canvas_to_parent: true,
        };

        let window = primary_window_override(&config, &options).unwrap();

        assert_eq!(window.canvas.as_deref(), Some("#viewer-canvas"));
        assert!(window.fit_canvas_to_parent);
    }

    #[test]
    fn width_and_height_config_still_override_primary_window_size() {
        let config = ViewerConfig {
            window_width: Some(640),
            window_height: Some(480),
            ..Default::default()
        };
        let options = ViewerAppOptions::default();

        let window = primary_window_override(&config, &options).unwrap();

        assert_eq!(window.resolution.width(), 640.0);
        assert_eq!(window.resolution.height(), 480.0);
        assert_eq!(window.canvas, None);
        assert!(!window.fit_canvas_to_parent);
    }
}
