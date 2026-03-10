use crate::components::FrameFace;
use crate::visuals::FaceVisual;
use bevy::{
    asset::RenderAssetUsages,
    light::{NotShadowCaster, NotShadowReceiver},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub fn render_faces(
    visuals: Vec<FaceVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    for visual in visuals {
        let Some(mesh) = build_face_mesh(&visual.vertices) else {
            continue;
        };

        let material = materials.add(StandardMaterial {
            base_color: visual.color,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            double_sided: true,
            perceptual_roughness: 0.65,
            metallic: 0.0,
            ..default()
        });

        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::default(),
            FrameFace,
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }
}

fn build_face_mesh(vertices: &[Vec3]) -> Option<Mesh> {
    if vertices.len() < 3 {
        return None;
    }

    let base = vertices[0];
    let edge_a = vertices[1] - base;
    let edge_b = vertices[2] - base;
    let normal = edge_a.cross(edge_b).normalize_or_zero();
    if normal == Vec3::ZERO {
        return None;
    }

    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| [v.x, v.y, v.z]).collect();
    let normals = vec![[normal.x, normal.y, normal.z]; vertices.len()];
    let mut indices = Vec::with_capacity((vertices.len() - 2) * 3);
    for i in 1..(vertices.len() - 1) {
        indices.push(0_u32);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}
