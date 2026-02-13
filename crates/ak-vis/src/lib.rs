pub mod visuals;
pub use visuals::{AtomVisual, AxisVisual, CellVisual, convert_cell, convert_structure, convert_axis};

pub mod color_palette;
pub use color_palette::{ColorPalette, ColorScheme, JMOL};

pub mod render;
pub use render::render_atoms;
