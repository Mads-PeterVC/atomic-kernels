use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct DisplayAtomIdentity {
    pub atom_index: usize,
    pub image_offset: [i32; 3],
}

#[derive(Component)]
pub struct FrameAtom;

#[derive(Component)]
pub struct FrameSelectionHighlight;

#[derive(Component)]
pub struct FrameMeasurementCue;

#[derive(Component)]
pub struct FrameBond;

#[derive(Component)]
pub struct FrameFace;

#[derive(Component)]
pub struct FrameCell;

#[derive(Component)]
pub struct FrameAxis;

#[derive(Component)]
pub struct ToggleableUI;

#[derive(Component)]
pub struct MainSceneCamera;

#[derive(Component)]
pub struct InspectorPanelRoot;

#[derive(Component)]
pub struct InspectorSelectionBody;

#[derive(Component)]
pub struct InspectorSelectionSection;

#[derive(Component)]
pub struct InspectorMeasurementBody;

#[derive(Component)]
pub struct InspectorMeasurementSection;

#[derive(Component)]
pub struct InspectorHintsContainer;

#[derive(Component)]
pub struct InspectorHintsToggle;

#[derive(Component)]
pub struct InspectorPanelSurface;

#[derive(Component)]
pub struct PlaybackPanelRoot;

#[derive(Component)]
pub struct PlaybackPanelSurface;

#[derive(Component)]
pub struct PlaybackStatusText;

#[derive(Component)]
pub struct PlaybackTitleText;

#[derive(Component)]
pub struct PlaybackFrameText;

#[derive(Component)]
pub struct PlaybackSpeedText;

#[derive(Component)]
pub struct PlaybackPlayPauseButton;

#[derive(Component)]
pub struct PlaybackPlayPauseIcon;

#[derive(Component)]
pub struct PlaybackStepBackButton;

#[derive(Component)]
pub struct PlaybackStepForwardButton;

#[derive(Component)]
pub struct PlaybackSpeedButton {
    pub index: usize,
}

#[derive(Component)]
pub struct PlaybackScrubberButton;

#[derive(Component)]
pub struct PlaybackScrubberFill;

#[derive(Component)]
pub struct PlaybackScrubberTrack;

#[derive(Component)]
pub struct PlaybackScrubberThumb;

#[derive(Component)]
pub struct MarqueeSelectionOverlay;

#[derive(Component)]
pub struct OrientationWidgetRoot;

#[derive(Component)]
pub struct OrientationWidgetCamera;

#[derive(Component)]
pub struct OrientationWidgetLetterStroke {
    pub direction: Vec3,
    pub start: Vec2,
    pub end: Vec2,
}

#[cfg(test)]
mod tests {
    use super::{
        DisplayAtomIdentity, FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace,
        FrameMeasurementCue, FrameSelectionHighlight, InspectorHintsContainer,
        InspectorHintsToggle, InspectorMeasurementBody, InspectorMeasurementSection,
        InspectorPanelRoot, InspectorPanelSurface, InspectorSelectionBody,
        InspectorSelectionSection, MainSceneCamera, MarqueeSelectionOverlay,
        OrientationWidgetCamera, OrientationWidgetLetterStroke, OrientationWidgetRoot,
        PlaybackFrameText, PlaybackPanelRoot, PlaybackPanelSurface, PlaybackPlayPauseButton,
        PlaybackPlayPauseIcon, PlaybackScrubberButton, PlaybackScrubberFill, PlaybackScrubberThumb,
        PlaybackScrubberTrack, PlaybackSpeedButton, PlaybackSpeedText, PlaybackStatusText,
        PlaybackStepBackButton, PlaybackStepForwardButton, PlaybackTitleText, ToggleableUI,
    };
    use bevy::prelude::*;

    #[test]
    fn component_markers_and_widget_strokes_attach_to_entities() {
        let mut world = World::new();

        let entity = world
            .spawn_empty()
            .insert((
                FrameAtom,
                FrameSelectionHighlight,
                FrameMeasurementCue,
                FrameBond,
                FrameFace,
                FrameCell,
                FrameAxis,
                ToggleableUI,
                MainSceneCamera,
            ))
            .insert(InspectorPanelRoot)
            .insert(InspectorSelectionSection)
            .insert(InspectorSelectionBody)
            .insert(InspectorMeasurementSection)
            .insert(InspectorMeasurementBody)
            .insert(InspectorHintsContainer)
            .insert(InspectorHintsToggle)
            .insert(InspectorPanelSurface)
            .insert(PlaybackPanelRoot)
            .insert(PlaybackPanelSurface)
            .insert(PlaybackStatusText)
            .insert(PlaybackTitleText)
            .insert(PlaybackFrameText)
            .insert(PlaybackSpeedText)
            .insert(PlaybackPlayPauseButton)
            .insert(PlaybackPlayPauseIcon)
            .insert(PlaybackStepBackButton)
            .insert(PlaybackStepForwardButton)
            .insert(PlaybackScrubberButton)
            .insert(PlaybackScrubberFill)
            .insert(PlaybackScrubberTrack)
            .insert(PlaybackScrubberThumb)
            .insert(MarqueeSelectionOverlay)
            .insert(OrientationWidgetRoot)
            .insert(OrientationWidgetCamera)
            .insert(PlaybackSpeedButton { index: 1 })
            .insert(DisplayAtomIdentity {
                atom_index: 3,
                image_offset: [1, 0, -1],
            })
            .insert(OrientationWidgetLetterStroke {
                direction: Vec3::Z,
                start: Vec2::new(-0.5, 0.0),
                end: Vec2::new(0.5, 0.0),
            })
            .id();

        let stroke = world.get::<OrientationWidgetLetterStroke>(entity).unwrap();
        assert_eq!(stroke.direction, Vec3::Z);
        assert_eq!(stroke.start, Vec2::new(-0.5, 0.0));
        assert_eq!(stroke.end, Vec2::new(0.5, 0.0));
        assert!(world.get::<FrameAtom>(entity).is_some());
        assert!(world.get::<FrameSelectionHighlight>(entity).is_some());
        assert!(world.get::<FrameMeasurementCue>(entity).is_some());
        assert!(world.get::<FrameBond>(entity).is_some());
        assert!(world.get::<FrameFace>(entity).is_some());
        assert!(world.get::<FrameCell>(entity).is_some());
        assert!(world.get::<FrameAxis>(entity).is_some());
        assert!(world.get::<ToggleableUI>(entity).is_some());
        assert!(world.get::<MainSceneCamera>(entity).is_some());
        assert!(world.get::<InspectorPanelRoot>(entity).is_some());
        assert!(world.get::<InspectorSelectionSection>(entity).is_some());
        assert!(world.get::<InspectorSelectionBody>(entity).is_some());
        assert!(world.get::<InspectorMeasurementSection>(entity).is_some());
        assert!(world.get::<InspectorMeasurementBody>(entity).is_some());
        assert!(world.get::<InspectorHintsContainer>(entity).is_some());
        assert!(world.get::<InspectorHintsToggle>(entity).is_some());
        assert!(world.get::<InspectorPanelSurface>(entity).is_some());
        assert!(world.get::<PlaybackPanelRoot>(entity).is_some());
        assert!(world.get::<PlaybackPanelSurface>(entity).is_some());
        assert!(world.get::<PlaybackStatusText>(entity).is_some());
        assert!(world.get::<PlaybackTitleText>(entity).is_some());
        assert!(world.get::<PlaybackFrameText>(entity).is_some());
        assert!(world.get::<PlaybackSpeedText>(entity).is_some());
        assert!(world.get::<PlaybackPlayPauseButton>(entity).is_some());
        assert!(world.get::<PlaybackPlayPauseIcon>(entity).is_some());
        assert!(world.get::<PlaybackStepBackButton>(entity).is_some());
        assert!(world.get::<PlaybackStepForwardButton>(entity).is_some());
        assert!(world.get::<PlaybackScrubberButton>(entity).is_some());
        assert!(world.get::<PlaybackScrubberFill>(entity).is_some());
        assert!(world.get::<PlaybackScrubberTrack>(entity).is_some());
        assert!(world.get::<PlaybackScrubberThumb>(entity).is_some());
        assert_eq!(world.get::<PlaybackSpeedButton>(entity).unwrap().index, 1);
        assert_eq!(
            world.get::<DisplayAtomIdentity>(entity).unwrap(),
            &DisplayAtomIdentity {
                atom_index: 3,
                image_offset: [1, 0, -1],
            }
        );
        assert!(world.get::<MarqueeSelectionOverlay>(entity).is_some());
        assert!(world.get::<OrientationWidgetRoot>(entity).is_some());
        assert!(world.get::<OrientationWidgetCamera>(entity).is_some());
    }
}
