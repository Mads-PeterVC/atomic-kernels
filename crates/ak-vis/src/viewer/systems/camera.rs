use crate::components::MainSceneCamera;
use crate::viewer::runtime::MainCameraRenderTarget;
use crate::viewer::{CameraState, ViewerConfig};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup_camera(
    mut commands: Commands,
    camera: Res<CameraState>,
    config: Res<ViewerConfig>,
    render_target: Option<Res<MainCameraRenderTarget>>,
) {
    if camera.radius <= 0.0 {
        return;
    }

    let is_headless = render_target.is_some();
    let mut camera = commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        PanOrbitCamera {
            yaw: Some(camera.yaw),
            pitch: Some(camera.pitch),
            radius: Some(camera.radius),
            focus: camera.focus,
            axis: [Vec3::X, Vec3::Y, Vec3::Z],
            orbit_smoothness: if is_headless { 0.0 } else { 0.1 },
            pan_smoothness: if is_headless { 0.0 } else { 0.02 },
            zoom_smoothness: if is_headless { 0.0 } else { 0.1 },
            ..default()
        },
    ));
    if let Some(render_target) = render_target {
        camera.insert(render_target.0.clone());
    }
    camera.insert(MainSceneCamera);
    if config.lighting.enable_fog {
        camera.insert(DistanceFog {
            color: config.color.background,
            directional_light_color: Color::WHITE,
            directional_light_exponent: 5.0,
            falloff: FogFalloff::Exponential { density: 0.0015 },
        });
    }
}

#[derive(Component)]
pub struct CameraLight;

pub fn setup_camera_light(mut commands: Commands, config: Res<ViewerConfig>) {
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.camera_illuminance,
            shadows_enabled: true,
            ..default()
        },
        CameraLight,
    ));
}

pub fn update_camera_light(
    cam_q: Query<(&GlobalTransform, &PanOrbitCamera)>,
    mut light_q: Query<&mut Transform, With<CameraLight>>,
) {
    let Ok((cam_gt, orbit)) = cam_q.single() else {
        return;
    };
    let Ok(mut light_transform) = light_q.single_mut() else {
        return;
    };

    let cam_pos = cam_gt.translation();
    let to_focus = (orbit.focus - cam_pos).normalize_or_zero();

    // Directional light shines along its -Z axis
    light_transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, to_focus);
}

pub fn advance_camera_motion(time: Res<Time>, mut camera: ResMut<CameraState>) {
    camera.tick_motion(time.delta_secs());
}

pub fn apply_camera_state(
    mut camera_state: ResMut<CameraState>,
    mut camera_query: Query<&mut PanOrbitCamera>,
) {
    if !camera_state.needs_apply {
        return;
    }

    let Ok(mut orbit) = camera_query.single_mut() else {
        return;
    };

    orbit.target_focus = camera_state.focus;
    orbit.target_radius = camera_state.radius;
    orbit.target_yaw = camera_state.yaw;
    orbit.target_pitch = camera_state.pitch;
    orbit.force_update = true;
    camera_state.needs_apply = false;
}
