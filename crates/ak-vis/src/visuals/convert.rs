use crate::{AtomVisual, AxisVisual, CellVisual, ColorScheme};
use ak_core::{PERIODIC_TABLE, StructureView};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn structure_vec3_to_world(vector: Vec3) -> Vec3 {
    // Rotate structure-space coordinates so chemistry-style z-up becomes Bevy-world y-up.
    Vec3::new(vector.x, vector.z, -vector.y)
}

pub fn structure_position_to_world(position: [f64; 3]) -> Vec3 {
    structure_vec3_to_world(Vec3::new(
        position[0] as f32,
        position[1] as f32,
        position[2] as f32,
    ))
}

pub fn convert_structure<C: ColorScheme>(view: &StructureView, scheme: &C) -> Vec<AtomVisual> {
    let mut visuals: Vec<AtomVisual> = Vec::with_capacity(view.len());
    for i in 0..view.positions.len() {
        let radius: f32 = 0.9 * PERIODIC_TABLE.get(view.numbers[i]).covalent_radius as f32;
        let color = scheme.color(view, i);
        let atom_visual = AtomVisual {
            position: structure_position_to_world(view.positions[i]).to_array(),
            color,
            radius,
        };
        visuals.push(atom_visual);
    }
    visuals
}

pub fn convert_cell(view: &StructureView, cell_color: Color) -> Vec<CellVisual> {
    let mut visuals: Vec<CellVisual> = Vec::new();

    let a = structure_vec3_to_world(Vec3::from_slice(view.cell.a().cast::<f32>().as_slice()));
    let b = structure_vec3_to_world(Vec3::from_slice(view.cell.b().cast::<f32>().as_slice()));
    let c = structure_vec3_to_world(Vec3::from_slice(view.cell.c().cast::<f32>().as_slice()));
    let origin = Vec3::ZERO;

    let edges = [
        (origin, a),
        (a, a + b),
        (a + b, b),
        (b, origin),
        (origin, c),
        (c, c + b),
        (c + b, b),
        (c + b, c + b + a),
        (c + b + a, b + a),
        (c + b + a, c + a),
        (c + a, c),
        (c + a, a),
    ];

    for edge in edges {
        let visual = CellVisual {
            corner_1: edge.0,
            corner_2: edge.1,
            color: cell_color,
        };
        visuals.push(visual);
    }

    visuals
}

pub fn convert_axis(view: &StructureView) -> Vec<AxisVisual> {
    let mut visuals: Vec<AxisVisual> = Vec::new();

    let a = structure_vec3_to_world(Vec3::from_slice(view.cell.a().cast::<f32>().as_slice()));
    let b = structure_vec3_to_world(Vec3::from_slice(view.cell.b().cast::<f32>().as_slice()));
    let c = structure_vec3_to_world(Vec3::from_slice(view.cell.c().cast::<f32>().as_slice()));

    let a_unit = a / a.length();
    let b_unit = b / b.length();
    let c_unit = c / c.length();

    visuals.push(AxisVisual {
        direction: a_unit,
        color: Color::srgb_u8(255, 0, 0),
        length: 1.0,
    });
    visuals.push(AxisVisual {
        direction: b_unit,
        color: Color::srgb_u8(0, 255, 0),
        length: 1.0,
    });
    visuals.push(AxisVisual {
        direction: c_unit,
        color: Color::srgb_u8(0, 0, 255),
        length: 1.0,
    });
    visuals
}
