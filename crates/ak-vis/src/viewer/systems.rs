mod camera;
mod commands;
mod lighting;
mod measurement;
mod rendering;
mod selection;
mod snapshot;

pub(crate) use camera::{
    advance_camera_motion, apply_camera_state, setup_camera, setup_camera_light,
    update_camera_light,
};
pub use commands::apply_viewer_commands;
pub use lighting::setup_lighting;
pub use rendering::{render_current_frame, rerender_if_dirty};
pub use selection::{
    MarqueeSelectionState, handle_atom_clicks, handle_marquee_selection, setup_marquee_overlay,
    sync_marquee_overlay,
};
pub use snapshot::sync_viewer_snapshot;

#[cfg(test)]
mod tests;
