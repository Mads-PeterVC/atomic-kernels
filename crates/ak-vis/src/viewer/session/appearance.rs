use crate::ScalarColorMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AppearanceChannel {
    Color,
    Metallic,
    PerceptualRoughness,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtomAppearanceRule {
    pub name: String,
    pub channel: AppearanceChannel,
    pub palette: Option<ScalarColorMap>,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BondScope {
    BothSelected,
    TouchSelection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BallAndStickStyle {
    pub atom_scale: f32,
    pub bond_radius: f32,
    pub bond_color: [f32; 4],
    pub bond_scope: BondScope,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderStyle {
    SpaceFilling,
    BallAndStick(BallAndStickStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderStyleRule {
    pub frame_index: usize,
    pub selection: Vec<bool>,
    pub style: RenderStyle,
}

impl BallAndStickStyle {
    pub fn bond_color(self) -> bevy::color::Color {
        bevy::color::Color::srgba(
            self.bond_color[0],
            self.bond_color[1],
            self.bond_color[2],
            self.bond_color[3],
        )
    }
}
