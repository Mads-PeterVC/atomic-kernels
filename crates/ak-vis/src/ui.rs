mod build;
mod shortcuts;
mod state;

#[cfg(test)]
mod tests;

pub use build::setup_ui;
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
