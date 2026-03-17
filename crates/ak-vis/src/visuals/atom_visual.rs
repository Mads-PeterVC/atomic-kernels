use bevy::color::Color;

pub struct AtomVisual {
    pub atom_index: usize,
    pub position: [f32; 3],
    pub color: Color,
    pub radius: f32,
}

impl AtomVisual {
    pub fn new(atom_index: usize, position: [f64; 3], color: Color, radius: f32) -> Self {
        let position_f32: [f32; 3] = [position[0] as f32, position[1] as f32, position[2] as f32];
        AtomVisual {
            atom_index,
            position: position_f32,
            color,
            radius,
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
