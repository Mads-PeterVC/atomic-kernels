use bevy::prelude::*;
use crate::components::ToggleableUI;

// System to toggle visibility
pub fn toggle_ui_visibility(
    mut query: Query<&mut Visibility, With<ToggleableUI>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyU) {
        for mut visibility in query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}