use crate::{AtomVisual, AxisVisual, CellVisual, ColorScheme};
use ak_core::{PERIODIC_TABLE, StructureView};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn convert_structure<C: ColorScheme>(view: &StructureView, scheme: &C) -> Vec<AtomVisual> {
    let mut visuals: Vec<AtomVisual> = Vec::with_capacity(view.len());
    for i in 0..view.positions.len() {
        let radius: f32 = PERIODIC_TABLE.get(view.numbers[i]).covalent_radius as f32;
        let color = scheme.color(&view, i);
        let atom_visual = AtomVisual::new(view.positions[i], color, radius);
        visuals.push(atom_visual);
    }
    visuals
}

pub fn convert_cell(view: &StructureView, cell_color: Color) -> Vec<CellVisual> {
    let mut visuals: Vec<CellVisual> = Vec::new();

    let a = Vec3::new(
        view.cell.m[0][0] as f32,
        view.cell.m[0][1] as f32,
        view.cell.m[0][2] as f32,
    );
    let b = Vec3::new(
        view.cell.m[1][0] as f32,
        view.cell.m[1][1] as f32,
        view.cell.m[1][2] as f32,
    );
    let c = Vec3::new(
        view.cell.m[2][0] as f32,
        view.cell.m[2][1] as f32,
        view.cell.m[2][2] as f32,
    );
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
            color: cell_color
        };
        visuals.push(visual);
    }

    visuals
}

pub fn convert_axis(view: &StructureView) -> Vec<AxisVisual> {
    let mut visuals: Vec<AxisVisual> = Vec::new();

    let a = Vec3::from_slice(&view.cell.m[0].map(|f| f as f32));
    let b = Vec3::from_slice(&view.cell.m[1].map(|f| f as f32));
    let c = Vec3::from_slice(&view.cell.m[2].map(|f| f as f32));

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
