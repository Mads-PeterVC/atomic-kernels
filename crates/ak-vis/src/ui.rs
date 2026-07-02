mod build;
mod playback;
mod shortcuts;
mod state;

#[cfg(test)]
mod tests;

pub use build::setup_ui;
pub(crate) use build::setup_viewer_ui;
pub use playback::{
    PlaybackState, handle_playback_buttons, set_camera_input_enabled, sync_playback_camera,
    sync_playback_slider_value, sync_playback_state, sync_playback_text, sync_playback_visibility,
};
pub use state::{
    InspectorState, MeasurementStatus, SelectedAtomSummary, derive_inspector_state,
    sync_inspector_camera, sync_inspector_state, sync_inspector_text, toggle_hints_visibility,
};

use bevy::prelude::Color;

pub(super) const PANEL_BACKGROUND: Color = Color::srgba(0.07, 0.08, 0.10, 0.58);
pub(super) const SECTION_BACKGROUND: Color = Color::srgba(0.10, 0.11, 0.14, 0.54);
pub(super) const PANEL_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);
pub(super) const BODY_COLOR: Color = Color::srgb(0.76, 0.79, 0.83);
pub(super) const ACCENT_COLOR: Color = Color::srgb(0.72, 0.86, 0.96);
pub(super) const KEYCAP_BACKGROUND: Color = Color::srgba(0.19, 0.21, 0.25, 0.82);
pub(super) const KEYCAP_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);
pub(super) const KEYCAP_TEXT: Color = Color::srgb(0.89, 0.91, 0.95);
pub(super) const HINT_LABEL_COLOR: Color = Color::srgb(0.70, 0.74, 0.79);
pub(super) const BUTTON_BACKGROUND: Color = Color::srgba(0.17, 0.19, 0.23, 0.86);
pub(super) const BUTTON_HOVER_BACKGROUND: Color = Color::srgba(0.23, 0.26, 0.31, 0.90);
pub(super) const BUTTON_ACTIVE_BACKGROUND: Color = Color::srgba(0.32, 0.44, 0.52, 0.96);
