use crate::AtomMaterial;
use bevy::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtomIdentity {
    pub atom_index: usize,
    pub image_offset: [i32; 3],
}

pub struct AtomVisual {
    pub atom_identity: AtomIdentity,
    pub position: [f32; 3],
    pub color: Color,
    pub material: AtomMaterial,
    pub radius: f32,
}

impl AtomVisual {
    pub fn new(atom_index: usize, position: [f64; 3], color: Color, radius: f32) -> Self {
        Self::new_with_identity(
            AtomIdentity {
                atom_index,
                image_offset: [0, 0, 0],
            },
            position,
            color,
            radius,
        )
    }

    pub fn new_with_identity(
        atom_identity: AtomIdentity,
        position: [f64; 3],
        color: Color,
        radius: f32,
    ) -> Self {
        let position_f32: [f32; 3] = [position[0] as f32, position[1] as f32, position[2] as f32];
        AtomVisual {
            atom_identity,
            position: position_f32,
            color,
            material: AtomMaterial::default(),
            radius,
        }
    }

    pub fn atom_index(&self) -> usize {
        self.atom_identity.atom_index
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
