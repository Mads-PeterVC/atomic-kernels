use crate::AtomVisual;
use crate::components::AtomIndex;
use crate::components::FrameAtom;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn render_atoms(
    visuals: Vec<AtomVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    quality: u32,
) {
    // Group atoms by (radius, color) to enable instancing
    // Using bits representation for radius as HashMap key (f32 doesn't implement Hash)
    let mut atom_groups: HashMap<(u32, [u8; 4]), Vec<(usize, Vec3)>> = HashMap::new();

    for atom in visuals.iter() {
        let radius_bits = atom.radius.to_bits();
        // Use color as bytes for HashMap key
        let color_key = [
            (atom.color.to_srgba().red * 255.0) as u8,
            (atom.color.to_srgba().green * 255.0) as u8,
            (atom.color.to_srgba().blue * 255.0) as u8,
            (atom.color.to_srgba().alpha * 255.0) as u8,
        ];

        let key = (radius_bits, color_key);
        atom_groups
            .entry(key)
            .or_default()
            .push((atom.atom_index, Vec3::new(atom.x(), atom.y(), atom.z())));
    }

    // Create shared meshes and materials for each unique atom type
    for ((radius_bits, color_bytes), positions) in atom_groups.iter() {
        let radius = f32::from_bits(*radius_bits);
        let color = Color::srgba(
            color_bytes[0] as f32 / 255.0,
            color_bytes[1] as f32 / 255.0,
            color_bytes[2] as f32 / 255.0,
            color_bytes[3] as f32 / 255.0,
        );

        // Create ONE mesh for this atom type (shared by all instances)
        let shared_mesh = meshes.add(
            Sphere::new(radius)
                .mesh()
                .ico(quality)
                .expect("Failed to create sphere mesh"),
        );

        // Create ONE material for this atom type (shared by all instances)
        let shared_material = materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.0,
            perceptual_roughness: 0.4,
            ..default()
        });

        // Spawn all atoms of this type - they share the same mesh handle,
        // so Bevy will automatically instance them in a single draw call
        for (atom_index, position) in positions {
            commands.spawn((
                Mesh3d(shared_mesh.clone()),
                MeshMaterial3d(shared_material.clone()),
                Transform::from_translation(*position),
                FrameAtom,
                AtomIndex(*atom_index),
            ));
        }
    }
}
