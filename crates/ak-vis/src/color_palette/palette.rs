use ak_core::StructureView;
use ak_core::geometry::AtomicNumber;
use bevy::color::Color;

pub struct ColorPalette {
    pub colors: [Option<Color>; 110],
    pub fallback: Color,
}

pub trait ColorScheme {
    fn color(&self, view: &StructureView, i: usize) -> Color;
}

impl ColorPalette {
    pub fn get(&self, atomic_number: AtomicNumber) -> Color {
        match self.colors[atomic_number.get() as usize] {
            Some(color) => color,
            None => self.fallback,
        }
    }
}

impl ColorScheme for ColorPalette {
    fn color(&self, view: &StructureView, i: usize) -> Color {
        self.get(view.numbers[i])
    }
}
