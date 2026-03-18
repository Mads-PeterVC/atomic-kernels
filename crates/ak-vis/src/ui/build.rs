use bevy::picking::prelude::Pickable;
use bevy::prelude::*;

use crate::components::{
    InspectorMeasurementBody, InspectorMeasurementSection, InspectorPanelRoot,
    InspectorPanelSurface, InspectorSelectionBody, InspectorSelectionSection, ToggleableUI,
};

use super::shortcuts::spawn_hints_section;
use super::{
    ACCENT_COLOR, BODY_COLOR, PANEL_BACKGROUND, PANEL_BORDER, SECTION_BACKGROUND,
};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/RobotoMono-VariableFont_wght.ttf");
    let symbol_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");

    commands
        .spawn_empty()
        .insert(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            padding: UiRect::all(px(18)),
            ..default()
        })
        .insert(ZIndex(20))
        .insert(Pickable::IGNORE)
        .insert(ToggleableUI)
        .insert(InspectorPanelRoot)
        .with_children(|parent| {
            parent
                .spawn_empty()
                .insert(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    width: px(280),
                    max_width: percent(28),
                    min_width: px(220),
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(16)),
                    ..default()
                })
                .insert(BackgroundColor(PANEL_BACKGROUND))
                .insert(BorderColor::all(PANEL_BORDER))
                .insert(Pickable::IGNORE)
                .insert(InspectorPanelSurface)
                .with_children(|panel| {
                    spawn_hints_section(panel, &font, &symbol_font);
                    spawn_section(
                        panel,
                        &font,
                        "Selection",
                        "No atoms selected.\nClick an atom to inspect it.",
                        Some(InspectorSelectionSection),
                        Some(InspectorSelectionBody),
                    );
                    spawn_section(
                        panel,
                        &font,
                        "Measurement",
                        "Select 2 atoms for a distance or 3 atoms for an angle.",
                        Some(InspectorMeasurementSection),
                        Some(InspectorMeasurementBody),
                    );
                });
        });
}

fn spawn_section<S: Component, M: Component>(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    title: &str,
    body: &str,
    section_marker: Option<S>,
    marker: Option<M>,
) {
    let mut entity = parent.spawn_empty();
    entity
        .insert(Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            width: percent(100),
            padding: UiRect::all(px(10)),
            border_radius: BorderRadius::all(px(12)),
            ..default()
        })
        .insert(BackgroundColor(SECTION_BACKGROUND))
        .insert(Pickable::IGNORE)
        .with_children(|section| {
            section.spawn(section_title_bundle(title, font));
            let mut entity = section.spawn(section_body_bundle(body, font));
            if let Some(marker) = marker {
                entity.insert(marker);
            }
        });
    if let Some(section_marker) = section_marker {
        entity.insert(section_marker);
    }
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

fn section_body_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
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
