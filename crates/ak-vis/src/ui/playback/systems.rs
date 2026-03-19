use bevy::ecs::query::QueryFilter;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{CoreSliderDragState, SliderRange, SliderValue};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::components::{
    MainSceneCamera, PlaybackFrameText, PlaybackPanelRoot, PlaybackPlayPauseButton,
    PlaybackPlayPauseIcon, PlaybackScrubberButton, PlaybackScrubberFill, PlaybackScrubberThumb,
    PlaybackSpeedButton, PlaybackSpeedText, PlaybackTitleText,
};
use crate::viewer::{MarqueeSelectionState, ViewerState};

use super::state::PlaybackState;
use crate::ui::{BUTTON_ACTIVE_BACKGROUND, BUTTON_BACKGROUND, BUTTON_HOVER_BACKGROUND};

pub fn sync_playback_state(viewer: Res<ViewerState>, mut playback: ResMut<PlaybackState>) {
    let next_total = viewer.trajectory_len();
    playback.current_frame = viewer.current;
    playback.total_frames = next_total;
    playback.follow_tail = viewer.follow_tail;

    if next_total <= 1 && playback.is_playing {
        playback.is_playing = false;
    }
}

pub fn sync_playback_visibility(
    playback: Res<PlaybackState>,
    mut roots: Query<&mut Node, With<PlaybackPanelRoot>>,
) {
    let display = if playback.total_frames <= 1 {
        Display::None
    } else {
        Display::Flex
    };

    for mut node in &mut roots {
        node.display = display;
    }
}

pub fn sync_playback_camera(
    mut commands: Commands,
    main_camera: Query<Entity, With<MainSceneCamera>>,
    roots: Query<(Entity, Option<&UiTargetCamera>), With<PlaybackPanelRoot>>,
) {
    let Ok(main_camera) = main_camera.single() else {
        return;
    };

    for (entity, target_camera) in roots.iter() {
        if target_camera.is_none() {
            commands.entity(entity).insert(UiTargetCamera(main_camera));
        }
    }
}

pub fn sync_playback_text(
    mut commands: Commands,
    playback: Res<PlaybackState>,
    mut title_texts: Query<
        &mut Text,
        (
            With<PlaybackTitleText>,
            Without<PlaybackFrameText>,
            Without<PlaybackSpeedText>,
            Without<PlaybackPlayPauseIcon>,
        ),
    >,
    mut frame_texts: Query<
        &mut Text,
        (
            With<PlaybackFrameText>,
            Without<PlaybackTitleText>,
            Without<PlaybackSpeedText>,
            Without<PlaybackPlayPauseIcon>,
        ),
    >,
    mut speed_texts: Query<
        &mut Text,
        (
            With<PlaybackSpeedText>,
            Without<PlaybackTitleText>,
            Without<PlaybackFrameText>,
            Without<PlaybackPlayPauseIcon>,
        ),
    >,
    mut play_pause_icons: Query<
        &mut Text,
        (
            With<PlaybackPlayPauseIcon>,
            Without<PlaybackTitleText>,
            Without<PlaybackFrameText>,
            Without<PlaybackSpeedText>,
        ),
    >,
    mut scrubber_fill: Query<
        &mut Node,
        (With<PlaybackScrubberFill>, Without<PlaybackScrubberThumb>),
    >,
    mut scrubber_thumb: Query<
        &mut Node,
        (With<PlaybackScrubberThumb>, Without<PlaybackScrubberFill>),
    >,
    slider: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
        ),
        With<PlaybackScrubberButton>,
    >,
    button_children: Query<&Children>,
    mut button_texts: Query<
        &mut Text,
        (
            Without<PlaybackTitleText>,
            Without<PlaybackFrameText>,
            Without<PlaybackSpeedText>,
            Without<PlaybackPlayPauseIcon>,
        ),
    >,
    play_buttons: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (With<Button>, With<PlaybackPlayPauseButton>),
    >,
    speed_buttons: Query<
        (
            Entity,
            &PlaybackSpeedButton,
            &Interaction,
            &mut BackgroundColor,
        ),
        (With<Button>, Without<PlaybackPlayPauseButton>),
    >,
) {
    if let Ok(mut text) = title_texts.single_mut() {
        *text = Text::new("Trajectory");
    }
    if let Ok(mut text) = frame_texts.single_mut() {
        *text = Text::new(format!(
            "Frame {} / {}",
            playback.current_frame.saturating_add(1),
            playback.total_frames.max(1)
        ));
    }
    if let Ok(mut text) = speed_texts.single_mut() {
        *text = Text::new(format!("{} playback", playback.current_speed().label));
    }
    if let Ok(mut icon) = play_pause_icons.single_mut() {
        *icon = Text::new(if playback.is_playing { "⏸" } else { "▶" });
    }

    if let Ok((entity, slider_value, slider_range, _hovered, drag_state)) = slider.single() {
        let max = playback.total_frames.saturating_sub(1) as f32;
        let expected_range = SliderRange::new(0.0, max.max(0.0));
        if *slider_range != expected_range {
            commands.entity(entity).insert(expected_range);
        }
        let expected_value = playback.current_frame as f32;
        if !drag_state.dragging && (slider_value.0 - expected_value).abs() > f32::EPSILON {
            commands.entity(entity).insert(SliderValue(expected_value));
        }

        let fraction = slider_range.thumb_position(slider_value.0);
        if let Ok(mut fill) = scrubber_fill.single_mut() {
            fill.width = percent(fraction * 100.0);
        }
        if let Ok(mut thumb) = scrubber_thumb.single_mut() {
            thumb.left = percent(fraction * 100.0);
        }
    }

    for (entity, interaction, mut background) in play_buttons {
        *background = button_background(playback.is_playing, *interaction);
        let _ = entity;
    }

    for (entity, speed, interaction, mut background) in speed_buttons {
        let is_active = speed.index == playback.speed_index;
        *background = button_background(is_active, *interaction);
        if let Some(preset) = PlaybackState::speed_presets().get(speed.index) {
            update_button_label(entity, &button_children, &mut button_texts, preset.label);
        }
    }
}

pub fn handle_playback_buttons(
    mut interactions: ParamSet<(
        Query<
            &Interaction,
            (
                Changed<Interaction>,
                With<Button>,
                With<PlaybackPlayPauseButton>,
            ),
        >,
        Query<
            &Interaction,
            (
                Changed<Interaction>,
                With<Button>,
                With<crate::components::PlaybackStepBackButton>,
                Without<PlaybackPlayPauseButton>,
            ),
        >,
        Query<
            &Interaction,
            (
                Changed<Interaction>,
                With<Button>,
                With<crate::components::PlaybackStepForwardButton>,
                Without<PlaybackPlayPauseButton>,
                Without<crate::components::PlaybackStepBackButton>,
            ),
        >,
        Query<
            (&Interaction, &PlaybackSpeedButton),
            (
                Changed<Interaction>,
                With<Button>,
                Without<PlaybackPlayPauseButton>,
                Without<crate::components::PlaybackStepBackButton>,
                Without<crate::components::PlaybackStepForwardButton>,
            ),
        >,
    )>,
    mut viewer: ResMut<ViewerState>,
    mut playback: ResMut<PlaybackState>,
) {
    if interactions
        .p0()
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        playback.is_playing = !playback.is_playing;
    }

    if interactions
        .p1()
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        viewer.step_frame(-1);
    }

    if interactions
        .p2()
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        let advanced = viewer.step_frame(1);
        if !advanced && !viewer.follow_tail {
            playback.is_playing = false;
        }
    }

    for (interaction, speed) in interactions.p3().iter() {
        if *interaction == Interaction::Pressed {
            playback.set_speed_index(speed.index);
        }
    }
}

pub fn sync_playback_slider_value(
    slider: Query<&SliderValue, (Changed<SliderValue>, With<PlaybackScrubberButton>)>,
    mut viewer: ResMut<ViewerState>,
) {
    let Ok(value) = slider.single() else {
        return;
    };
    let max = viewer.trajectory_len().saturating_sub(1) as f32;
    let frame = value.0.round().clamp(0.0, max) as usize;
    viewer.set_current_frame_index(frame);
}

pub fn set_camera_input_enabled(
    sliders: Query<&CoreSliderDragState, With<PlaybackScrubberButton>>,
    marquee: Option<Res<MarqueeSelectionState>>,
    mut cameras: Query<&mut PanOrbitCamera>,
) {
    let dragging = sliders.iter().any(|drag| drag.dragging);
    let marquee_active = marquee
        .as_ref()
        .is_some_and(|marquee| marquee.is_tracking());
    for mut camera in &mut cameras {
        camera.enabled = !(dragging || marquee_active);
    }
}

fn update_button_label<F: QueryFilter>(
    entity: Entity,
    button_children: &Query<&Children>,
    button_texts: &mut Query<&mut Text, F>,
    label: &str,
) {
    let Ok(children) = button_children.get(entity) else {
        return;
    };
    for child in children.iter() {
        if let Ok(mut text) = button_texts.get_mut(child) {
            *text = Text::new(label);
            break;
        }
    }
}

fn button_background(active: bool, interaction: Interaction) -> BackgroundColor {
    match interaction {
        Interaction::Pressed => BUTTON_ACTIVE_BACKGROUND.into(),
        Interaction::Hovered if active => BUTTON_ACTIVE_BACKGROUND.into(),
        Interaction::Hovered => BUTTON_HOVER_BACKGROUND.into(),
        Interaction::None if active => BUTTON_ACTIVE_BACKGROUND.into(),
        Interaction::None => BUTTON_BACKGROUND.into(),
    }
}
