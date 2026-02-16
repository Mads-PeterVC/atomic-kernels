use crate::viewer::app::ViewerTrajectory;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn default_radius_focus(viewer: Res<ViewerTrajectory>, mut cam_query: Query<&mut PanOrbitCamera>) {
    let view = viewer.traj.view(viewer.current);
    let cell_midpoint = Vec3::from_array(view.cell.reduced(0.5, 0.5, 0.5).map(|f| f as f32));

    let radius = [view.cell.a(), view.cell.b(), view.cell.c()]
        .iter()
        .map(|vec| vec.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap() as f32;

    let Ok(mut orbit) = cam_query.single_mut() else {
        return;
    };

    orbit.target_focus = cell_midpoint;
    orbit.target_radius = 2.5 * radius;
}
