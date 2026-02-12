use bevy::color::Color;

pub struct AtomVisual {
    pub position: [f32; 3],
    pub color: Color,
    pub radius: f32,
}

impl AtomVisual {
    pub fn new(position: [f64; 3], color: Color, radius: f32) -> Self {
        let position_f32: [f32; 3] = [position[0] as f32, position[1] as f32, position[2] as f32];
        AtomVisual {
            position: position_f32,
            color: color,
            radius: radius,
        }
    }

    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }

    pub fn z(&self) -> f32 {
        self.position[2]
    }
}
