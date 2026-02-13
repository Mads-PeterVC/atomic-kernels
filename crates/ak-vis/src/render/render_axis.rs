use crate::visuals::AxisVisual;
use bevy::prelude::*;

pub fn render_axis(
    visuals: Vec<AxisVisual>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    for visual in visuals {
        let dir = visual.direction.normalize_or_zero();
        let rot = if dir.length_squared() > 0.0 {
            Quat::from_rotation_arc(Vec3::Y, dir)
        } else {
            Quat::IDENTITY
        };

        let cyl_mesh = Cylinder::new(0.1, visual.length);

        commands.spawn((
            Mesh3d(meshes.add(cyl_mesh)),
            MeshMaterial3d(materials.add(visual.color)),
            Transform::from_rotation(rot).with_translation(dir * (visual.length * 0.5)),
        ));
    }
}
