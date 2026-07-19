use crate::viewer::ViewerConfig;
use bevy::prelude::*;

pub fn setup_lighting(
    mut commands: Commands,
    mut ambient_light: ResMut<GlobalAmbientLight>,
    config: Res<ViewerConfig>,
) {
    ambient_light.brightness = config.lighting.ambient_brightness;

    // KEY light (main): above + to the side + from front
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.key_illuminance,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            -0.8, // yaw
            -0.6, // pitch
            0.0,
        )),
    ));

    // FILL light (soft): opposite side, weaker, no shadows
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.fill_illuminance,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 1.6, -0.2, 0.0)),
    ));

    // RIM / BACK light: behind to pop silhouettes a bit
    commands.spawn((
        DirectionalLight {
            illuminance: config.lighting.back_illuminance,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -2.6, -0.3, 0.0)),
    ));
}
