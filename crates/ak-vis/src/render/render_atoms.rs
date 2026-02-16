use crate::AtomVisual;
use crate::components::FrameAtom;
use bevy::prelude::*;

pub fn render_atoms(
    visuals: Vec<AtomVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    for atom in visuals.iter() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(atom.radius))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: atom.color,
                metallic: 0.0,
                perceptual_roughness: 0.4,
                ..default()
            })),
            Transform::from_xyz(atom.x(), atom.y(), atom.z()),
            FrameAtom
        ));
    }
}
