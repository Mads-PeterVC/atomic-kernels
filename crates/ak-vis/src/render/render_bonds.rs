use crate::components::FrameBond;
use crate::visuals::BondVisual;
use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

pub fn render_bonds(
    visuals: Vec<BondVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let cyl_mesh = Cylinder::new(1.0, 2.0);

    for visual in visuals {
        commands.spawn((
            Mesh3d(meshes.add(cyl_mesh)),
            MeshMaterial3d(materials.add(visual.color)),
            transform_cylinder_between(visual.start, visual.end, visual.radius),
            FrameBond,
            NotShadowCaster,
            NotShadowReceiver,
        ));
    }
}

fn transform_cylinder_between(p0: Vec3, p1: Vec3, radius: f32) -> Transform {
    let d = p1 - p0;
    let len = d.length();
    let mid = (p0 + p1) * 0.5;
    let dir = d / len;
    let rot = Quat::from_rotation_arc(Vec3::Y, dir);

    Transform {
        translation: mid,
        rotation: rot,
        scale: Vec3::new(radius, len * 0.5, radius),
    }
}
