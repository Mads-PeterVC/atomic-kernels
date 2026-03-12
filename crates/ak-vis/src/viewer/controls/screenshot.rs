use bevy::{
    prelude::*,
    render::view::screenshot::{Capturing, Screenshot, save_to_disk},
    window::{CursorIcon, SystemCursorIcon},
};

pub fn screenshot_on_spacebar(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

pub fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    window: Single<Entity, With<Window>>,
) {
    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(*window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{screenshot_on_spacebar, screenshot_saving};
    use bevy::{
        ecs::system::SystemState,
        prelude::*,
        render::view::screenshot::{Capturing, Screenshot},
        window::{CursorIcon, SystemCursorIcon},
    };

    #[test]
    fn screenshot_on_spacebar_spawns_primary_window_capture() {
        let mut world = World::new();
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::Space);
        world.insert_resource(input);

        let mut system_state: SystemState<(
            Commands,
            Res<ButtonInput<KeyCode>>,
            Local<u32>,
        )> = SystemState::new(&mut world);

        let (commands, input, counter) = system_state.get_mut(&mut world);
        screenshot_on_spacebar(commands, input, counter);
        system_state.apply(&mut world);

        assert_eq!(world.query::<&Screenshot>().iter(&world).count(), 1);
    }

    #[test]
    fn screenshot_saving_updates_window_cursor_state() {
        let mut world = World::new();
        let window = world.spawn(Window::default()).id();

        let mut system_state: SystemState<(
            Commands,
            Query<Entity, With<Capturing>>,
            Single<Entity, With<Window>>,
        )> = SystemState::new(&mut world);

        {
            let (commands, capturing, window) = system_state.get_mut(&mut world);
            screenshot_saving(commands, capturing, window);
        }
        system_state.apply(&mut world);
        assert!(world.get::<CursorIcon>(window).is_none());

        world.spawn(Capturing);
        {
            let (commands, capturing, window) = system_state.get_mut(&mut world);
            screenshot_saving(commands, capturing, window);
        }
        system_state.apply(&mut world);

        let icon = world.get::<CursorIcon>(window).unwrap();
        assert_eq!(*icon, CursorIcon::from(SystemCursorIcon::Progress));
    }
}
