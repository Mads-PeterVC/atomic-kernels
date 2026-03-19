mod inspector;
mod playback_panel;

use bevy::prelude::*;

use crate::viewer::ViewerConfig;

use super::{ACCENT_COLOR, BODY_COLOR};

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<ViewerConfig>,
) {
    let font = asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf");
    let symbol_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");

    inspector::spawn_inspector_panel(&mut commands, &font, &symbol_font);
    playback_panel::spawn_playback_panel(&mut commands, &font, &symbol_font, config.as_ref());
}

pub(super) fn section_title_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 12.0,
            ..default()
        },
        TextColor(ACCENT_COLOR),
    )
}

pub(super) fn section_body_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 11.0,
            ..default()
        },
        TextColor(BODY_COLOR),
    )
}
