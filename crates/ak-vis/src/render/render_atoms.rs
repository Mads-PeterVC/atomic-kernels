use crate::AtomVisual;
use crate::components::FrameAtom;
use crate::viewer::picking::update_material_on;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

pub fn render_atoms(
    visuals: Vec<AtomVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let hover_matl = materials.add(Color::from(CYAN_300));

    for atom in visuals.iter() {
        let atom_material = materials.add(StandardMaterial {
            base_color: atom.color,
            metallic: 0.0,
            perceptual_roughness: 0.4,
            ..default()
        });

        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(atom.radius).mesh().ico(7).unwrap())),
                MeshMaterial3d(atom_material.clone()),
                Transform::from_xyz(atom.x(), atom.y(), atom.z()),
                FrameAtom,
            ))
            .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            .observe(update_material_on::<Pointer<Out>>(atom_material.clone()));
    }
}
