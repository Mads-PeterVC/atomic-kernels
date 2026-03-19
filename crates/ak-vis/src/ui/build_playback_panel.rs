use bevy::picking::hover::Hovered;
use bevy::picking::prelude::Pickable;
use bevy::prelude::*;
use bevy::ui_widgets::{
    Slider, SliderRange, SliderThumb, SliderValue, TrackClick, observe, slider_self_update,
};

use crate::components::{
    PlaybackFrameText, PlaybackPanelRoot, PlaybackPanelSurface, PlaybackPlayPauseButton,
    PlaybackPlayPauseIcon, PlaybackScrubberButton, PlaybackScrubberFill,
    PlaybackScrubberThumb, PlaybackScrubberTrack, PlaybackSpeedButton, PlaybackSpeedText,
    PlaybackStatusText, PlaybackStepBackButton, PlaybackStepForwardButton, PlaybackTitleText,
    ToggleableUI,
};
use crate::viewer::ViewerConfig;

use super::super::playback::PlaybackState;
use super::super::playback::widgets::{
    button_text_bundle, status_frame_text_bundle, status_speed_text_bundle,
    status_title_text_bundle,
};
use crate::ui::{
    ACCENT_COLOR, BODY_COLOR, BUTTON_BACKGROUND, KEYCAP_BORDER, PANEL_BACKGROUND, PANEL_BORDER,
};

pub(super) fn spawn_playback_panel(
    commands: &mut Commands,
    font: &Handle<Font>,
    symbol_font: &Handle<Font>,
    _config: &ViewerConfig,
) {
    commands
        .spawn_empty()
        .insert(Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(18),
            ..default()
        })
        .insert(ZIndex(20))
        .insert(ToggleableUI)
        .insert(PlaybackPanelRoot)
        .with_children(|parent| {
            parent
                .spawn_empty()
                .insert(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(8),
                    width: px(240),
                    max_width: percent(24),
                    min_width: px(195),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(16)),
                    ..default()
                })
                .insert(BackgroundColor(PANEL_BACKGROUND))
                .insert(BorderColor::all(PANEL_BORDER))
                .insert(PlaybackPanelSurface)
                .with_children(|panel| {
                    panel
                        .spawn((
                            Node {
                                width: percent(100),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(2),
                                padding: UiRect::axes(px(2), px(1)),
                                ..default()
                            },
                            PlaybackStatusText,
                        ))
                        .with_children(|status| {
                            status.spawn(status_title_text_bundle(
                                "Trajectory",
                                font,
                                PlaybackTitleText,
                            ));
                            status.spawn(status_frame_text_bundle(
                                "Frame 1 / 1",
                                font,
                                PlaybackFrameText,
                            ));
                            status.spawn(status_speed_text_bundle(
                                "2x playback",
                                font,
                                PlaybackSpeedText,
                            ));
                        });
                    panel
                        .spawn_empty()
                        .insert(Node {
                            width: percent(100),
                            align_items: AlignItems::Center,
                            column_gap: px(6),
                            ..default()
                        })
                        .with_children(|row| {
                            spawn_button(row, font, "<", PlaybackStepBackButton, 1.0);
                            spawn_icon_button(
                                row,
                                symbol_font,
                                "▶",
                                PlaybackPlayPauseButton,
                                PlaybackPlayPauseIcon,
                                2.0,
                            );
                            spawn_button(row, font, ">", PlaybackStepForwardButton, 1.0);
                        });
                    panel
                        .spawn((
                            Node {
                                width: percent(100),
                                height: px(24),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            PlaybackScrubberButton,
                            Hovered::default(),
                            Slider {
                                track_click: TrackClick::Snap,
                            },
                            SliderValue(0.0),
                            SliderRange::new(0.0, 0.0),
                            observe(slider_self_update),
                        ))
                        .with_children(|slider| {
                            slider.spawn((
                                Node {
                                    width: percent(100),
                                    position_type: PositionType::Absolute,
                                    top: px(9),
                                    height: px(4),
                                    border: UiRect::all(px(1)),
                                    border_radius: BorderRadius::all(px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(BUTTON_BACKGROUND),
                                BorderColor::all(KEYCAP_BORDER),
                                Pickable::IGNORE,
                                PlaybackScrubberTrack,
                            ));
                            slider
                                .spawn((
                                    Node {
                                        display: Display::Flex,
                                        position_type: PositionType::Absolute,
                                        left: px(0),
                                        right: px(12),
                                        top: px(0),
                                        height: px(24),
                                        ..default()
                                    },
                                    Pickable::IGNORE,
                                ))
                                .with_children(|overlay| {
                                    overlay.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: px(0),
                                            top: px(10),
                                            width: percent(0),
                                            height: px(2),
                                            border_radius: BorderRadius::all(px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(ACCENT_COLOR),
                                        Pickable::IGNORE,
                                        PlaybackScrubberFill,
                                    ));
                                    overlay.spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: percent(0),
                                            top: px(4),
                                            width: px(12),
                                            height: px(16),
                                            border_radius: BorderRadius::all(px(999)),
                                            ..default()
                                        },
                                        BackgroundColor(ACCENT_COLOR),
                                        BorderColor::all(KEYCAP_BORDER),
                                        Pickable::IGNORE,
                                        SliderThumb,
                                        PlaybackScrubberThumb,
                                    ));
                                });
                        });
                    panel
                        .spawn_empty()
                        .insert(Node {
                            width: percent(100),
                            align_items: AlignItems::Center,
                            column_gap: px(6),
                            ..default()
                        })
                        .with_children(|row| {
                            for (index, preset) in PlaybackState::speed_presets().iter().enumerate()
                            {
                                row.spawn((
                                    Button,
                                    Node {
                                        flex_grow: 1.0,
                                        height: px(28),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border_radius: BorderRadius::all(px(10)),
                                        ..default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND),
                                    PlaybackSpeedButton { index },
                                ))
                                .with_child(button_text_bundle(preset.label, font));
                            }
                        });
                });
        });
}

fn spawn_button<M: Component>(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    label: &str,
    marker: M,
    flex_grow: f32,
) {
    parent
        .spawn((
            Button,
            Node {
                flex_grow,
                height: px(30),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(px(8), px(4)),
                border_radius: BorderRadius::all(px(10)),
                ..default()
            },
            BackgroundColor(BUTTON_BACKGROUND),
            marker,
        ))
        .with_child(button_text_bundle(label, font));
}

fn spawn_icon_button<M: Component, I: Component>(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    glyph: &str,
    marker: M,
    icon_marker: I,
    flex_grow: f32,
) {
    parent
        .spawn((
            Button,
            Node {
                flex_grow,
                height: px(30),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(px(8), px(4)),
                border_radius: BorderRadius::all(px(10)),
                ..default()
            },
            BackgroundColor(BUTTON_BACKGROUND),
            marker,
        ))
        .with_child((
            Text::new(glyph),
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(BODY_COLOR),
            icon_marker,
        ));
}
