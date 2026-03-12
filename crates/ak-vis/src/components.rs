use bevy::prelude::*;

#[derive(Component)]
pub struct FrameAtom;

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
        FrameAtom, FrameAxis, FrameBond, FrameCell, FrameFace, MainSceneCamera,
        OrientationWidgetCamera, OrientationWidgetLetterStroke, OrientationWidgetRoot,
        ToggleableUI,
    };
    use bevy::prelude::*;

    #[test]
    fn component_markers_and_widget_strokes_attach_to_entities() {
        let mut world = World::new();

        let entity = world
            .spawn((
                FrameAtom,
                FrameBond,
                FrameFace,
                FrameCell,
                FrameAxis,
                ToggleableUI,
                MainSceneCamera,
                OrientationWidgetRoot,
                OrientationWidgetCamera,
                OrientationWidgetLetterStroke {
                    direction: Vec3::Z,
                    start: Vec2::new(-0.5, 0.0),
                    end: Vec2::new(0.5, 0.0),
                },
            ))
            .id();

        let stroke = world.get::<OrientationWidgetLetterStroke>(entity).unwrap();
        assert_eq!(stroke.direction, Vec3::Z);
        assert_eq!(stroke.start, Vec2::new(-0.5, 0.0));
        assert_eq!(stroke.end, Vec2::new(0.5, 0.0));
        assert!(world.get::<FrameAtom>(entity).is_some());
        assert!(world.get::<FrameBond>(entity).is_some());
        assert!(world.get::<FrameFace>(entity).is_some());
        assert!(world.get::<FrameCell>(entity).is_some());
        assert!(world.get::<FrameAxis>(entity).is_some());
        assert!(world.get::<ToggleableUI>(entity).is_some());
        assert!(world.get::<MainSceneCamera>(entity).is_some());
        assert!(world.get::<OrientationWidgetRoot>(entity).is_some());
        assert!(world.get::<OrientationWidgetCamera>(entity).is_some());
    }
}
