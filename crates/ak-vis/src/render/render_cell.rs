use crate::visuals::CellVisual;
use crate::components::FrameCell;

use bevy::prelude::*;

pub fn render_cell(
    visuals: Vec<CellVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let cyl_mesh = Cylinder::new(1.0, 2.0);

    for visual in visuals {
        commands.spawn((
            Mesh3d(meshes.add(cyl_mesh.clone())),
            MeshMaterial3d(materials.add(visual.color)),
            transform_cylinder_between(visual.corner_1, visual.corner_2, 0.025),
            FrameCell,
        ));
    }
}

fn transform_cylinder_between(p0: Vec3, p1: Vec3, radius: f32) -> Transform {
    let d = p1 - p0;
    let len = d.length();
    let mid = (p0 + p1) * 0.5;

    // Bevy's Cylinder is aligned along +Y by default (in most setups).
    // So rotate +Y to match the direction d.
    let dir = d / len;
    let rot = Quat::from_rotation_arc(Vec3::Y, dir);

    Transform {
        translation: mid,
        rotation: rot,
        scale: Vec3::new(radius, len * 0.5, radius), // y scale = half-length if cylinder height is 2.0; adjust if yours differs
        ..default()
    }
}
