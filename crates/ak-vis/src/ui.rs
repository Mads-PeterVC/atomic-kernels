use bevy::{
    prelude::*,
};

pub fn setup_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: px(24),
                bottom: px(24),
                width: px(270),
                padding: UiRect::all(px(16)),
                border_radius: BorderRadius::all(px(12)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12).with_alpha(0.85)),
            BorderColor::all(Color::WHITE.with_alpha(0.15)),
            ZIndex(10),
        ))
        .insert(children![text_bundle("Keybindings")]);
}

fn text_bundle(text: &str) -> impl Bundle {
    let bundle = (
        Text::new(text),
        // Set the justification of the Text
        Underline,
        TextLayout::new_with_justify(Justify::Center),
        // Set the style of the Node itself.
        // Node {
        //     position_type: PositionType::Absolute,
        //     bottom: px(5),
        //     left: px(10),
        //     ..default()
        // },
    );
    bundle
}
