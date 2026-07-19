use crate::viewer::runtime::CommandReceiver;
use crate::viewer::{CameraState, ViewerState};
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn apply_viewer_commands(
    mut viewer: ResMut<ViewerState>,
    mut camera: ResMut<CameraState>,
    receiver: Res<CommandReceiver>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    let Some(receiver) = receiver.0.as_ref() else {
        return;
    };

    let Ok(receiver) = receiver.lock() else {
        return;
    };

    loop {
        match receiver.try_recv() {
            Ok(command) => {
                camera.apply_command(viewer.as_ref(), &command);
                let outcome = viewer.apply_command(command);
                if outcome.should_close {
                    app_exit_events.write(AppExit::Success);
                    break;
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => break,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
        }
    }
}
