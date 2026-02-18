use bevy::prelude::*;

use crate::components::ToggleableUI;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf");

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: px(24),
                bottom: px(24),
                width: px(200),
                padding: UiRect::all(px(16)),
                border_radius: BorderRadius::all(px(12)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12).with_alpha(0.85)),
            BorderColor::all(Color::WHITE.with_alpha(0.15)),
            ZIndex(10),
            ToggleableUI,
        ))
        .insert(children![text_bundle("Keybindings", &font)]);

    commands.spawn((
        Text::new("Some text"),
        ToggleableUI,
        TextLayout::new_with_justify(Justify::Right),
        TextFont {
            font: font.clone(),
            font_size: 12.0,
            ..default()
        },
    ));
}

fn text_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    
    (
        Text::new(text),
        // Set the justification of the Text
        Underline,
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font: font.clone(),
            ..default()
        }, // Set the style of the Node itself.
           // Node {
           //     position_type: PositionType::Absolute,
           //     bottom: px(5),
           //     left: px(10),
           //     ..default()
           // },
    )
}
