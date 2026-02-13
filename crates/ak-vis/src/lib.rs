pub mod visuals;
pub use visuals::{
    AtomVisual, AxisVisual, CellVisual, convert_axis, convert_cell, convert_structure,
};

pub mod color_palette;
pub use color_palette::{ColorPalette, ColorScheme, JMOL};

pub mod render;
pub use render::render_atoms;
