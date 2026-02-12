pub mod atom_visual;
pub use atom_visual::{AtomVisual, convert_structure};

pub mod color_palette;
pub use color_palette::{ColorPalette, ColorScheme, JMOL};

pub mod render;
pub use render::render_atoms;
