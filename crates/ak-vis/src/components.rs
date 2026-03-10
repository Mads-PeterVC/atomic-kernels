use bevy::prelude::*;

#[derive(Component)]
pub struct FrameAtom;

#[derive(Component)]
pub struct FrameBond;

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
