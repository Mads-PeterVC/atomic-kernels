use ak_core::StructureView;
use ak_core::geometry::AtomicNumber;
use bevy::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AtomMaterial {
    pub metallic: f32,
    pub perceptual_roughness: f32,
}

impl AtomMaterial {
    pub const fn new(metallic: f32, perceptual_roughness: f32) -> Self {
        Self {
            metallic,
            perceptual_roughness,
        }
    }
}

pub const DEFAULT_ATOM_MATERIAL: AtomMaterial = AtomMaterial::new(0.0, 0.4);

impl Default for AtomMaterial {
    fn default() -> Self {
        DEFAULT_ATOM_MATERIAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorPalette {
    pub colors: [Option<Color>; 110],
    pub materials: [Option<AtomMaterial>; 110],
    pub fallback: Color,
    pub fallback_material: AtomMaterial,
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

    pub fn material(&self, atomic_number: AtomicNumber) -> AtomMaterial {
        match self.materials[atomic_number.get() as usize] {
            Some(material) => material,
            None => self.fallback_material,
        }
    }
}

impl ColorScheme for ColorPalette {
    fn color(&self, view: &StructureView, i: usize) -> Color {
        self.get(view.numbers[i])
    }
}
