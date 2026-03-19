pub mod palette;
pub use palette::{AtomMaterial, ColorPalette, ColorScheme, DEFAULT_ATOM_MATERIAL};

pub mod jmol;
pub use jmol::{JMOL, JMOL_METALLIC};

pub mod scalar;
pub use scalar::ScalarColorMap;

pub fn named_palette(name: &str) -> &'static ColorPalette {
    match name {
        "jmol-metallic" => &JMOL_METALLIC,
        _ => &JMOL,
    }
}
