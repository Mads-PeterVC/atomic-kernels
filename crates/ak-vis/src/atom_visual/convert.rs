use crate::{AtomVisual, ColorScheme};
use ak_core::{PERIODIC_TABLE, StructureView};

pub fn convert_structure<C: ColorScheme>(view: StructureView, scheme: &C) -> Vec<AtomVisual> {
    let mut visuals: Vec<AtomVisual> = Vec::with_capacity(view.len());
    for i in 0..view.positions.len() {
        let radius: f32 = PERIODIC_TABLE.get(view.numbers[i]).covalent_radius as f32;
        let color = scheme.color(&view, i);
        let atom_visual = AtomVisual::new(view.positions[i], color, radius);
        visuals.push(atom_visual);
    }
    visuals
}
