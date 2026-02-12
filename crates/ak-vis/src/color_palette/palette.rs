use ak_core::geometry::AtomicNumber;
use bevy::color::Color;

pub struct ColorPalette {
    pub colors: [Option<Color>; 110],
    pub fallback: Color,
}

impl ColorPalette {
    pub fn get(&self, atomic_number: AtomicNumber) -> Color {
        match self.colors[atomic_number.get() as usize] {
            Some(color) => color,
            None => self.fallback,
        }
    }
}
