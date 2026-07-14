use crate::viewer::ViewerState;
use crate::viewer::runtime::SharedViewerSnapshot;
use bevy::prelude::*;

pub fn sync_viewer_snapshot(viewer: Res<ViewerState>, snapshot: Res<SharedViewerSnapshot>) {
    let Ok(mut snapshot) = snapshot.0.lock() else {
        return;
    };
    snapshot.current_frame = viewer.current;
    snapshot.selection = viewer.selection.clone();
    snapshot.image_selection = viewer.image_selection.clone();
    snapshot.supercell = viewer.supercell;
}
