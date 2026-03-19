use bevy::picking::prelude::Pickable;
use bevy::prelude::*;

use crate::components::{InspectorHintsContainer, InspectorHintsToggle};

use super::build::section_title_bundle;
use super::{HINT_LABEL_COLOR, KEYCAP_BACKGROUND, KEYCAP_BORDER, KEYCAP_TEXT, SECTION_BACKGROUND};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShortcutToken {
    Key(&'static str),
    Separator(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShortcutHint {
    tokens: &'static [ShortcutToken],
    pub(crate) label: &'static str,
}

const CLICK_REPLACE_SELECTION_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("click")];
const SHIFT_CLICK_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("click"),
];
const SHIFT_DRAG_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("drag"),
];
const FRAME_STEP_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("A"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("D"),
];
const ZOOM_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("W"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("S"),
];
const ORBIT_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("◀"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("▶"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("▲"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("▼"),
];
const SNAP_VIEW_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("X"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Y"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Z"),
];
const SCREENSHOT_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("Space")];
const TOGGLE_INSPECTOR_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("U")];
const TOGGLE_HINTS_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("H")];
const REPEAT_A_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("1"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("1"),
];
const REPEAT_B_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("2"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("2"),
];
const REPEAT_C_TOKENS: &[ShortcutToken] = &[
    ShortcutToken::Key("3"),
    ShortcutToken::Separator("/"),
    ShortcutToken::Key("Shift"),
    ShortcutToken::Separator("+"),
    ShortcutToken::Key("3"),
];
const TOGGLE_SUPERCELL_DISTINCTION_TOKENS: &[ShortcutToken] = &[ShortcutToken::Key("0")];

const SHORTCUT_HINTS: &[ShortcutHint] = &[
    ShortcutHint {
        tokens: CLICK_REPLACE_SELECTION_TOKENS,
        label: "replace selection",
    },
    ShortcutHint {
        tokens: SHIFT_CLICK_TOKENS,
        label: "toggle atom",
    },
    ShortcutHint {
        tokens: SHIFT_DRAG_TOKENS,
        label: "marquee replace",
    },
    ShortcutHint {
        tokens: FRAME_STEP_TOKENS,
        label: "step frames",
    },
    ShortcutHint {
        tokens: ZOOM_TOKENS,
        label: "zoom",
    },
    ShortcutHint {
        tokens: ORBIT_TOKENS,
        label: "orbit camera",
    },
    ShortcutHint {
        tokens: SNAP_VIEW_TOKENS,
        label: "snap to axes",
    },
    ShortcutHint {
        tokens: SCREENSHOT_TOKENS,
        label: "save screenshot",
    },
    ShortcutHint {
        tokens: TOGGLE_INSPECTOR_TOKENS,
        label: "toggle UI",
    },
    ShortcutHint {
        tokens: TOGGLE_HINTS_TOKENS,
        label: "toggle keybindings",
    },
    ShortcutHint {
        tokens: REPEAT_A_TOKENS,
        label: "repeat a",
    },
    ShortcutHint {
        tokens: REPEAT_B_TOKENS,
        label: "repeat b",
    },
    ShortcutHint {
        tokens: REPEAT_C_TOKENS,
        label: "repeat c",
    },
    ShortcutHint {
        tokens: TOGGLE_SUPERCELL_DISTINCTION_TOKENS,
        label: "toggle image ghosting",
    },
];

pub(crate) fn shortcut_hints() -> &'static [ShortcutHint] {
    SHORTCUT_HINTS
}

pub(super) fn spawn_hints_section(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    symbol_font: &Handle<Font>,
) {
    parent
        .spawn_empty()
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
            section
                .spawn_empty()
                .insert(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(6),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .with_children(|header| {
                    header.spawn(section_title_bundle("Keybindings", font));
                    header.spawn(hint_action_bundle("(", font));
                    spawn_keycap(header, font, symbol_font, "H");
                    header.spawn((
                        Text::new(")"),
                        TextFont {
                            font: font.clone(),
                            font_size: 11.0,
                            ..default()
                        },
                        TextColor(HINT_LABEL_COLOR),
                        InspectorHintsToggle,
                    ));
                });
            section
                .spawn_empty()
                .insert(Node {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4),
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .insert(InspectorHintsContainer)
                .with_children(|container| {
                    for hint in shortcut_hints() {
                        spawn_hint_row(container, font, symbol_font, hint);
                    }
                });
        });
}

fn spawn_hint_row(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    symbol_font: &Handle<Font>,
    hint: &ShortcutHint,
) {
    parent
        .spawn_empty()
        .insert(Node {
            align_items: AlignItems::Center,
            column_gap: px(8),
            ..default()
        })
        .insert(Pickable::IGNORE)
        .with_children(|row| {
            row.spawn_empty()
                .insert(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(4),
                    ..default()
                })
                .insert(Pickable::IGNORE)
                .with_children(|tokens_parent| {
                    for token in hint.tokens {
                        match token {
                            ShortcutToken::Key(text) => {
                                spawn_keycap(tokens_parent, font, symbol_font, text)
                            }
                            ShortcutToken::Separator(text) => {
                                tokens_parent.spawn(hint_separator_bundle(text, font));
                            }
                        }
                    }
                });
            row.spawn(hint_action_bundle(hint.label, font));
        });
}

fn keycap_font<'a>(
    font: &'a Handle<Font>,
    symbol_font: &'a Handle<Font>,
    text: &str,
) -> &'a Handle<Font> {
    match text {
        "◀" | "▶" | "▲" | "▼" => symbol_font,
        _ => font,
    }
}

fn is_symbol_keycap(text: &str) -> bool {
    matches!(text, "◀" | "▶" | "▲" | "▼")
}

fn symbol_keycap_text_node(text: &str) -> Node {
    let mut node = Node::default();
    match text {
        "◀" => {
            node.position_type = PositionType::Relative;
            node.top = px(1.5);
        }
        "▶" => {
            node.position_type = PositionType::Relative;
            node.top = px(1.5);
            node.left = px(0.5);
        }
        "▲" => {
            node.position_type = PositionType::Relative;
            node.top = px(0.75);
        }
        "▼" => {
            node.position_type = PositionType::Relative;
            node.top = px(1.5);
        }
        _ => {}
    }
    node
}

fn spawn_keycap(
    parent: &mut ChildSpawnerCommands<'_>,
    font: &Handle<Font>,
    symbol_font: &Handle<Font>,
    text: &str,
) {
    let is_symbol = is_symbol_keycap(text);
    parent
        .spawn_empty()
        .insert(Node {
            min_width: if is_symbol { px(22) } else { px(0) },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: if is_symbol {
                UiRect::axes(px(5), px(2))
            } else {
                UiRect::axes(px(7), px(3))
            },
            border: UiRect::all(px(1)),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        })
        .insert(BackgroundColor(KEYCAP_BACKGROUND))
        .insert(BorderColor::all(KEYCAP_BORDER))
        .insert(Pickable::IGNORE)
        .with_children(|keycap| {
            keycap.spawn((
                if is_symbol {
                    symbol_keycap_text_node(text)
                } else {
                    Node::default()
                },
                Text::new(text),
                TextFont {
                    font: keycap_font(font, symbol_font, text).clone(),
                    font_size: if is_symbol { 12.0 } else { 10.5 },
                    ..default()
                },
                TextColor(KEYCAP_TEXT),
            ));
        });
}

fn hint_separator_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 10.5,
            ..default()
        },
        TextColor(HINT_LABEL_COLOR),
    )
}

fn hint_action_bundle(text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 11.0,
            ..default()
        },
        TextColor(HINT_LABEL_COLOR),
    )
}
