use bevy::color::Color;
use bevy::prelude::Vec3;

pub struct BondVisual {
    pub start: Vec3,
    pub end: Vec3,
    pub color: Color,
    pub radius: f32,
}
