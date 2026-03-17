use crate::viewer::ViewerConfig;
use crate::viewer::session::{CameraState, ViewerCommand, ViewerSnapshot, ViewerState};
use crate::viewer::systems::{
    advance_camera_motion, apply_camera_state, apply_viewer_commands, render_current_frame,
    rerender_if_dirty, setup_camera, setup_camera_light, setup_lighting, sync_viewer_snapshot,
    update_camera_light,
};
use crate::ui::InspectorState;
use ak_core::Trajectory;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use std::sync::{Arc, Mutex, mpsc};

#[derive(Resource)]
pub(crate) struct CommandReceiver(pub Option<Mutex<mpsc::Receiver<ViewerCommand>>>);

#[derive(Resource, Clone)]
pub(crate) struct MainCameraRenderTarget(pub RenderTarget);

#[derive(Resource, Clone)]
pub(crate) struct SharedViewerSnapshot(pub Arc<Mutex<ViewerSnapshot>>);

pub(crate) fn asset_root() -> String {
    format!("{}/assets", env!("CARGO_MANIFEST_DIR"))
}

pub(crate) fn configure_shared_app(
    app: &mut App,
    trajectory: Trajectory,
    config: ViewerConfig,
    mut receiver: Option<mpsc::Receiver<ViewerCommand>>,
    snapshot: Arc<Mutex<ViewerSnapshot>>,
) {
    let mut viewer_state = ViewerState::new(trajectory, config.initial_frame);
    let mut camera_state = CameraState::new(&viewer_state);

    if let Some(receiver_ref) = receiver.as_mut() {
        while let Ok(command) = receiver_ref.try_recv() {
            camera_state.apply_command(&viewer_state, &command);
            let _ = viewer_state.apply_command(command);
        }
    }

    app.insert_resource(ClearColor(config.color.background))
        .insert_resource(viewer_state)
        .insert_resource(camera_state)
        .insert_resource(config)
        .insert_resource(InspectorState::default())
        .insert_resource(SharedViewerSnapshot(snapshot))
        .insert_resource(CommandReceiver(receiver.map(Mutex::new)))
        .add_systems(
            Startup,
            (
                setup_lighting,
                setup_camera,
                render_current_frame,
                setup_camera_light,
                sync_viewer_snapshot,
            ),
        )
        .add_systems(
            Update,
            (
                apply_viewer_commands,
                advance_camera_motion,
                apply_camera_state,
                update_camera_light,
                rerender_if_dirty,
                sync_viewer_snapshot,
            ),
        );
}
