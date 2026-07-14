use super::helpers::*;
use crate::ScalarColorMap;
use crate::viewer::session::{
    AppearanceChannel, AtomAppearanceRule, BallAndStickStyle, BondScope, RenderStyle,
    ViewerCommand, ViewerState,
};
use ak_core::Trajectory;
#[test]
fn set_atom_scalars_stores_current_frame_values() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::SetAtomScalars {
        name: "energy".to_string(),
        values: vec![1.0, 2.0],
        frame_index: None,
    });

    assert_eq!(
        state.atom_scalars["energy"][0].as_ref().unwrap(),
        &vec![1.0, 2.0]
    );
}

#[test]
fn map_appearance_by_scalar_replaces_existing_channel_rules() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "energy".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Viridis),
        min: None,
        max: None,
        append: false,
    });

    assert_eq!(
        state.atom_appearance_rules,
        vec![AtomAppearanceRule {
            name: "energy".to_string(),
            channel: AppearanceChannel::Color,
            palette: Some(ScalarColorMap::Viridis),
            min: None,
            max: None,
        }]
    );
}

#[test]
fn append_appearance_rule_preserves_existing_rules_for_same_channel() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "energy".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Viridis),
        min: None,
        max: None,
        append: false,
    });
    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "charge".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Plasma),
        min: Some(-1.0),
        max: Some(1.0),
        append: true,
    });

    assert_eq!(state.atom_appearance_rules.len(), 2);
    assert_eq!(state.atom_appearance_rules[0].name, "energy");
    assert_eq!(state.atom_appearance_rules[1].name, "charge");
}

#[test]
fn replacing_one_channel_preserves_other_channels() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "energy".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Viridis),
        min: None,
        max: None,
        append: false,
    });
    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "charge".to_string(),
        channel: AppearanceChannel::Metallic,
        palette: None,
        min: Some(0.0),
        max: Some(1.0),
        append: false,
    });
    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "height".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Plasma),
        min: None,
        max: None,
        append: false,
    });

    assert_eq!(state.atom_appearance_rules.len(), 2);
    assert_eq!(
        state.atom_appearance_rules[0].channel,
        AppearanceChannel::Metallic
    );
    assert_eq!(state.atom_appearance_rules[1].name, "height");
}

#[test]
fn resetting_one_channel_preserves_other_channels() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "energy".to_string(),
        channel: AppearanceChannel::Color,
        palette: Some(ScalarColorMap::Viridis),
        min: None,
        max: None,
        append: false,
    });
    state.apply_command(ViewerCommand::MapAppearanceByScalar {
        name: "charge".to_string(),
        channel: AppearanceChannel::PerceptualRoughness,
        palette: None,
        min: Some(0.0),
        max: Some(1.0),
        append: false,
    });

    state.apply_command(ViewerCommand::ResetAtomAppearance {
        channel: Some(AppearanceChannel::Color),
    });

    assert_eq!(state.atom_appearance_rules.len(), 1);
    assert_eq!(
        state.atom_appearance_rules[0].channel,
        AppearanceChannel::PerceptualRoughness
    );
}

#[test]
fn set_render_style_stores_selection_rule() {
    let mut state = ViewerState::new(Trajectory::new(vec![test_structure(0.0)]), 0);

    state.apply_command(ViewerCommand::SetRenderStyle {
        style: RenderStyle::BallAndStick(BallAndStickStyle {
            atom_scale: 0.45,
            bond_radius: 0.08,
            bond_color: [0.7, 0.7, 0.7, 1.0],
            bond_scope: BondScope::TouchSelection,
        }),
        selection: vec![true, false],
        frame_index: None,
        append: false,
    });

    assert_eq!(state.render_style_rules.len(), 1);
    assert_eq!(state.render_style_rules[0].selection, vec![true, false]);
    assert_eq!(state.render_style_rules[0].frame_index, 0);
    assert!(matches!(
        state.render_style_rules[0].style,
        RenderStyle::BallAndStick(BallAndStickStyle {
            bond_scope: BondScope::TouchSelection,
            ..
        })
    ));
}
