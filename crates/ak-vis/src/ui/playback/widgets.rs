use bevy::prelude::*;

use crate::ui::{ACCENT_COLOR, BODY_COLOR};

pub(in crate::ui) fn button_text_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
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

pub(in crate::ui) fn status_title_text_bundle<M: Component>(
    text: &str,
    font: &Handle<Font>,
    marker: M,
) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 10.0,
            ..default()
        },
        TextColor(ACCENT_COLOR),
        marker,
    )
}

pub(in crate::ui) fn status_frame_text_bundle<M: Component>(
    text: &str,
    font: &Handle<Font>,
    marker: M,
) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::WHITE),
        marker,
    )
}

pub(in crate::ui) fn status_speed_text_bundle<M: Component>(
    text: &str,
    font: &Handle<Font>,
    marker: M,
) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 10.0,
            ..default()
        },
        TextColor(BODY_COLOR),
        marker,
    )
}
