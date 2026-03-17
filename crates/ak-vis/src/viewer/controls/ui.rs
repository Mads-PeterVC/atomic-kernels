use crate::components::ToggleableUI;
use bevy::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::toggle_ui_visibility;
    use crate::components::ToggleableUI;
    use bevy::prelude::*;

    #[test]
    fn toggle_ui_visibility_flips_visibility_on_u_press() {
        let mut app = App::new();
        let entity = app
            .world_mut()
            .spawn((ToggleableUI, Visibility::Visible))
            .id();

        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyU);
        app.insert_resource(keys);
        app.add_systems(Update, toggle_ui_visibility);

        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(entity).unwrap(),
            Visibility::Hidden
        );

        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyU);
        app.insert_resource(keys);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(entity).unwrap(),
            Visibility::Visible
        );
    }
}
