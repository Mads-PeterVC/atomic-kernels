pub mod visuals;
pub use visuals::{
    AtomVisual, AxisVisual, CellVisual, convert_axis, convert_cell, convert_structure,
};

pub mod color_palette;
pub use color_palette::{ColorPalette, ColorScheme, JMOL, ScalarColorMap};

pub mod render;
pub use render::render_atoms;

pub mod viewer;
pub use viewer::{
    AtomColorRule, ViewerCommand, ViewerSessionClosed, ViewerSessionHandle, launch, run,
    run_default, run_prepared, run_structure, run_structure_default, run_with_session,
};

pub mod ui;

mod components;
