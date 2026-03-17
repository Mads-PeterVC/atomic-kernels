use bevy::prelude::*;

#[derive(Component)]
pub struct FrameAtom;

#[derive(Component)]
pub struct FrameSelectionHighlight;

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
pub struct InspectorHintsBody;

#[derive(Component)]
pub struct InspectorHintsToggle;

#[derive(Component)]
pub struct InspectorPanelSurface;

#[derive(Component)]
pub struct AtomIndex(pub usize);

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
        AtomIndex, FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, FrameSelectionHighlight,
        InspectorHintsBody, InspectorHintsToggle, InspectorMeasurementBody, InspectorPanelRoot,
        InspectorMeasurementSection, InspectorPanelSurface, InspectorSelectionBody,
        InspectorSelectionSection, MainSceneCamera, MarqueeSelectionOverlay,
        OrientationWidgetCamera, OrientationWidgetLetterStroke, OrientationWidgetRoot,
        ToggleableUI,
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
                FrameBond,
                FrameFace,
                FrameCell,
                FrameAxis,
                ToggleableUI,
                MainSceneCamera,
            ))
            .insert((
                InspectorPanelRoot,
                InspectorSelectionSection,
                InspectorSelectionBody,
                InspectorMeasurementSection,
                InspectorMeasurementBody,
                InspectorHintsBody,
                InspectorHintsToggle,
                InspectorPanelSurface,
                AtomIndex(3),
                MarqueeSelectionOverlay,
                OrientationWidgetRoot,
                OrientationWidgetCamera,
            ))
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
        assert!(world.get::<InspectorHintsBody>(entity).is_some());
        assert!(world.get::<InspectorHintsToggle>(entity).is_some());
        assert!(world.get::<InspectorPanelSurface>(entity).is_some());
        assert_eq!(world.get::<AtomIndex>(entity).unwrap().0, 3);
        assert!(world.get::<MarqueeSelectionOverlay>(entity).is_some());
        assert!(world.get::<OrientationWidgetRoot>(entity).is_some());
        assert!(world.get::<OrientationWidgetCamera>(entity).is_some());
    }
}
