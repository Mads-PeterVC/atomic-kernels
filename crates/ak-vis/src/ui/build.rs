#[path = "build_inspector.rs"]
mod inspector;
#[path = "build_playback_panel.rs"]
mod playback_panel;

use bevy::prelude::*;

use crate::viewer::{ViewerConfig, ViewerFonts};

use super::{ACCENT_COLOR, BODY_COLOR};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<ViewerConfig>) {
    let font = asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf");
    let symbol_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");

    spawn_ui(&mut commands, &font, &symbol_font, config.as_ref());
}

pub(crate) fn setup_viewer_ui(
    mut commands: Commands,
    fonts: Res<ViewerFonts>,
    config: Res<ViewerConfig>,
) {
    spawn_ui(&mut commands, &fonts.mono, &fonts.symbols, config.as_ref());
}

fn spawn_ui(
    commands: &mut Commands,
    font: &Handle<Font>,
    symbol_font: &Handle<Font>,
    config: &ViewerConfig,
) {
    inspector::spawn_inspector_panel(commands, font, symbol_font);
    playback_panel::spawn_playback_panel(commands, font, symbol_font, config);
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
