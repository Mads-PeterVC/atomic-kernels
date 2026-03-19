use ak_core::Structure;
use ak_vis::viewer::{RenderConfig, ViewerConfig};
use ak_vis::visuals::{BondVisual, FaceVisual};
use ak_vis::{
    AtomMaterial, AtomVisual, AxisVisual, ColorScheme, JMOL, JMOL_METALLIC, ScalarColorMap,
    named_palette, render, render_atoms,
};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

#[path = "../examples/config_scene.rs"]
#[allow(dead_code)]
mod config_scene;
#[path = "../examples/headless_scene.rs"]
#[allow(dead_code)]
mod headless_scene;
#[path = "../examples/relaxation_scene.rs"]
#[allow(dead_code)]
mod relaxation_scene;
#[path = "../examples/trajectory_scene.rs"]
#[allow(dead_code)]
mod trajectory_scene;

type RenderSystemState<'w, 's> = SystemState<(
    Commands<'w, 's>,
    ResMut<'w, Assets<StandardMaterial>>,
    ResMut<'w, Assets<Mesh>>,
)>;

fn example_structure() -> Structure {
    Structure::new(
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        vec![1, 8],
        [[4.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 4.0]],
        [false, false, false],
    )
}

#[test]
fn jmol_palette_and_color_scheme_return_expected_colors() {
    let structure = example_structure();
    let hydrogen = JMOL.get(structure.view().numbers[0]).to_srgba();
    let oxygen = JMOL.color(&structure.view(), 1).to_srgba();

    assert_eq!(
        (hydrogen.red, hydrogen.green, hydrogen.blue),
        (1.0, 1.0, 1.0)
    );
    assert!((oxygen.red - 1.0).abs() < 1e-6);
    assert!(oxygen.green < 0.1);
    assert!(oxygen.blue < 0.1);
}

#[test]
fn scalar_color_maps_clamp_and_interpolate() {
    let low = ScalarColorMap::Viridis.color(-1.0).to_srgba();
    let high = ScalarColorMap::Inferno.color(2.0).to_srgba();
    let sampled = ScalarColorMap::Sampled(vec![[0.0, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]])
        .color(0.5)
        .to_srgba();

    assert!(low.blue > 0.3);
    assert!(high.red > 0.9);
    assert!((sampled.red - 0.5).abs() < 1e-6);
    assert_eq!(sampled.alpha, 1.0);
}

#[test]
fn viewer_config_defaults_are_accessible_from_public_surface() {
    let config = ViewerConfig::default();
    let render = RenderConfig::default();

    assert_eq!(config.initial_frame, 0);
    assert_eq!(config.render.atom_palette, "jmol");
    assert!(config.render.show_cell);
    assert!(render.show_orientation_widget);
    assert_eq!(render.ico_subdiv, 4);
}

#[test]
fn named_palettes_expose_expected_atom_materials() {
    let structure = example_structure();
    let hydrogen = structure.view().numbers[0];

    assert_eq!(
        named_palette("jmol").material(hydrogen),
        AtomMaterial::default()
    );
    assert_eq!(
        named_palette("missing").material(hydrogen),
        JMOL.material(hydrogen)
    );
    assert_eq!(
        named_palette("jmol-metallic").material(hydrogen),
        JMOL_METALLIC.material(hydrogen)
    );
}

#[test]
fn render_atoms_uses_atom_visual_material_for_standard_material() {
    let mut world = World::new();
    world.init_resource::<Assets<StandardMaterial>>();
    world.init_resource::<Assets<Mesh>>();

    let mut system_state: RenderSystemState<'_, '_> = SystemState::new(&mut world);

    {
        let (mut commands, mut materials, mut meshes) = system_state.get_mut(&mut world);
        let mut metallic = AtomVisual::new(0, [0.0, 0.0, 0.0], Color::srgb(1.0, 0.8, 0.0), 0.5);
        metallic.material = AtomMaterial::new(0.9, 0.15);
        render_atoms(
            vec![metallic],
            &mut commands,
            &mut materials,
            &mut meshes,
            2,
        );
    }
    system_state.apply(&mut world);

    let materials = world.resource::<Assets<StandardMaterial>>();
    let created: Vec<&StandardMaterial> = materials.iter().map(|(_, material)| material).collect();

    assert_eq!(created.len(), 1);
    assert!((created[0].metallic - 0.9).abs() < 1e-6);
    assert!((created[0].perceptual_roughness - 0.15).abs() < 1e-6);
}

#[test]
fn render_helpers_spawn_expected_entities() {
    let mut world = World::new();
    world.init_resource::<Assets<StandardMaterial>>();
    world.init_resource::<Assets<Mesh>>();

    let mut system_state: RenderSystemState<'_, '_> = SystemState::new(&mut world);

    {
        let (mut commands, mut materials, mut meshes) = system_state.get_mut(&mut world);
        render_atoms(
            vec![
                AtomVisual::new(0, [0.0, 0.0, 0.0], Color::srgb(1.0, 0.0, 0.0), 0.5),
                AtomVisual::new(1, [1.0, 0.0, 0.0], Color::srgb(1.0, 0.0, 0.0), 0.5),
                AtomVisual::new(2, [0.0, 1.0, 0.0], Color::srgb(0.0, 0.0, 1.0), 0.25),
            ],
            &mut commands,
            &mut materials,
            &mut meshes,
            2,
        );
        render::render_axis(
            vec![AxisVisual {
                direction: Vec3::X,
                color: Color::srgb(0.0, 1.0, 0.0),
                length: 2.0,
            }],
            &mut commands,
            &mut materials,
            &mut meshes,
        );
        render::render_bonds(
            vec![BondVisual {
                start: Vec3::ZERO,
                end: Vec3::Y,
                color: Color::srgb(0.8, 0.8, 0.8),
                radius: 0.1,
            }],
            &mut commands,
            &mut materials,
            &mut meshes,
        );
        render::render_cell(
            vec![ak_vis::CellVisual {
                corner_1: Vec3::ZERO,
                corner_2: Vec3::new(0.0, 0.0, 2.0),
                color: Color::srgb(1.0, 1.0, 1.0),
            }],
            &mut commands,
            &mut materials,
            &mut meshes,
        );
        render::render_faces(
            vec![FaceVisual {
                vertices: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
                color: Color::srgba(1.0, 0.0, 0.0, 0.4),
            }],
            &mut commands,
            &mut materials,
            &mut meshes,
        );
    }
    system_state.apply(&mut world);

    assert_eq!(world.query::<&Mesh3d>().iter(&world).count(), 7);
    assert_eq!(world.query::<&Transform>().iter(&world).count(), 7);
}
