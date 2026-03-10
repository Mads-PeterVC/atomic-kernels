use bevy::camera::{ClearColorConfig, Viewport, visibility::RenderLayers};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{
    MainSceneCamera, OrientationWidgetCamera, OrientationWidgetLetterStroke,
    OrientationWidgetRoot,
};
use crate::viewer::ViewerConfig;
use crate::visuals::structure_vec3_to_world;

const ORIENTATION_WIDGET_LAYER: usize = 1;
const WIDGET_CAMERA_DISTANCE: f32 = 5.0;
const SHAFT_RADIUS: f32 = 0.18;
const SHAFT_LENGTH: f32 = 1.9;
const HEAD_RADIUS: f32 = 0.36;
const HEAD_HEIGHT: f32 = 0.78;
const LABEL_DISTANCE: f32 = 2.9;
const LETTER_SIZE: f32 = 0.45;
const LETTER_STROKE_RADIUS: f32 = 0.06;

#[derive(Clone, Copy)]
struct AxisSpec {
    direction: Vec3,
    color: Color,
    label: &'static str,
}

fn structure_axes() -> [AxisSpec; 3] {
    [
        AxisSpec {
            direction: structure_vec3_to_world(Vec3::X),
            color: Color::srgb(1.0, 0.0, 0.0),
            label: "X",
        },
        AxisSpec {
            direction: structure_vec3_to_world(Vec3::Y),
            color: Color::srgb(0.0, 1.0, 0.0),
            label: "Y",
        },
        AxisSpec {
            direction: structure_vec3_to_world(Vec3::Z),
            color: Color::srgb(0.0, 0.2, 1.0),
            label: "Z",
        },
    ]
}

pub fn widget_viewport(
    physical_width: u32,
    physical_height: u32,
    widget_size_px: u32,
    offset_x_px: u32,
    offset_y_px: u32,
) -> Viewport {
    let max_size_x = physical_width.saturating_sub(offset_x_px);
    let max_size_y = physical_height.saturating_sub(offset_y_px);
    let size = widget_size_px.min(max_size_x).min(max_size_y).max(1);
    let x = offset_x_px.min(physical_width.saturating_sub(size));
    let bottom_offset = offset_y_px.min(physical_height.saturating_sub(size));
    let y = physical_height
        .saturating_sub(size)
        .saturating_sub(bottom_offset);
    Viewport {
        physical_position: UVec2::new(x, y),
        physical_size: UVec2::splat(size),
        depth: 0.0..1.0,
    }
}

pub fn setup_orientation_widget(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    config: Res<ViewerConfig>,
) {
    if !config.render.show_orientation_widget {
        return;
    }

    let widget_layer = RenderLayers::layer(ORIENTATION_WIDGET_LAYER);
    let shaft_mesh = meshes.add(Cylinder::new(SHAFT_RADIUS, SHAFT_LENGTH));
    let head_mesh = meshes.add(Cone::new(HEAD_RADIUS, HEAD_HEIGHT));
    let stroke_mesh = meshes.add(Cylinder::new(1.0, 2.0));
    let viewport = windows.single().ok().map(|window| {
        widget_viewport(
            window.physical_width(),
            window.physical_height(),
            config.render.orientation_widget_size_px,
            config.render.orientation_widget_offset_x_px,
            config.render.orientation_widget_offset_y_px,
        )
    });

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: config.render.orientation_widget_camera_scale,
            ..OrthographicProjection::default_3d()
        }),
        Camera {
            order: 100,
            clear_color: ClearColorConfig::None,
            viewport,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, WIDGET_CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
        OrientationWidgetCamera,
        widget_layer.clone(),
    ));

    let root = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            OrientationWidgetRoot,
            widget_layer.clone(),
        ))
        .id();

    for axis in structure_axes() {
        let material = materials.add(StandardMaterial {
            base_color: axis.color,
            unlit: true,
            ..default()
        });
        spawn_widget_axis(
            &mut commands,
            root,
            axis.direction,
            shaft_mesh.clone(),
            head_mesh.clone(),
            material.clone(),
            widget_layer.clone(),
        );
        spawn_widget_label(
            &mut commands,
            axis.direction,
            axis.label,
            stroke_mesh.clone(),
            material,
            widget_layer.clone(),
        );
    }
}

fn spawn_widget_axis(
    commands: &mut Commands,
    root: Entity,
    direction: Vec3,
    shaft_mesh: Handle<Mesh>,
    head_mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    layer: RenderLayers,
) {
    let shaft_translation = direction * (SHAFT_LENGTH * 0.5);
    let head_translation = direction * (SHAFT_LENGTH + HEAD_HEIGHT * 0.5);
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Mesh3d(shaft_mesh),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(shaft_translation).with_rotation(rotation),
            layer.clone(),
        ));

        parent.spawn((
            Mesh3d(head_mesh),
            MeshMaterial3d(material),
            Transform::from_translation(head_translation).with_rotation(rotation),
            layer,
        ));
    });
}

fn spawn_widget_label(
    commands: &mut Commands,
    direction: Vec3,
    label: &'static str,
    stroke_mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    layer: RenderLayers,
) {
    let strokes = match label {
        "X" => vec![
            [Vec2::new(-0.5, -0.5), Vec2::new(0.5, 0.5)],
            [Vec2::new(-0.5, 0.5), Vec2::new(0.5, -0.5)],
        ],
        "Y" => vec![
            [Vec2::new(-0.5, 0.5), Vec2::new(0.0, 0.0)],
            [Vec2::new(0.5, 0.5), Vec2::new(0.0, 0.0)],
            [Vec2::new(0.0, 0.0), Vec2::new(0.0, -0.6)],
        ],
        "Z" => vec![
            [Vec2::new(-0.5, 0.5), Vec2::new(0.5, 0.5)],
            [Vec2::new(0.5, 0.5), Vec2::new(-0.5, -0.5)],
            [Vec2::new(-0.5, -0.5), Vec2::new(0.5, -0.5)],
        ],
        _ => Vec::new(),
    };

    for [start, end] in strokes {
        commands.spawn((
            Mesh3d(stroke_mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::default(),
            OrientationWidgetLetterStroke {
                direction,
                start,
                end,
            },
            layer.clone(),
        ));
    }
}

pub fn sync_orientation_widget(
    main_camera: Query<&GlobalTransform, (With<MainSceneCamera>, Without<OrientationWidgetRoot>)>,
    mut widget_root: Query<&mut Transform, With<OrientationWidgetRoot>>,
) {
    let Ok(main_camera) = main_camera.single() else {
        return;
    };
    let Ok(mut widget_root) = widget_root.single_mut() else {
        return;
    };

    widget_root.rotation = main_camera.rotation().inverse();
}

pub fn update_orientation_widget_viewport(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut widget_camera: Query<&mut Camera, With<OrientationWidgetCamera>>,
    mut projections: Query<&mut Projection, With<OrientationWidgetCamera>>,
    config: Res<ViewerConfig>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(mut camera) = widget_camera.single_mut() else {
        return;
    };

    let viewport = widget_viewport(
        window.physical_width(),
        window.physical_height(),
        config.render.orientation_widget_size_px,
        config.render.orientation_widget_offset_x_px,
        config.render.orientation_widget_offset_y_px,
    );
    camera.viewport = Some(viewport);

    if let Ok(mut projection) = projections.single_mut()
        && let Projection::Orthographic(orthographic) = &mut *projection
    {
        orthographic.scale = config.render.orientation_widget_camera_scale;
    }
}

pub fn sync_orientation_widget_letter_strokes(
    main_camera: Query<&GlobalTransform, (With<MainSceneCamera>, Without<OrientationWidgetRoot>)>,
    mut strokes: Query<(&OrientationWidgetLetterStroke, &mut Transform)>,
) {
    let Ok(main_camera) = main_camera.single() else {
        return;
    };

    let root_rotation = main_camera.rotation().inverse();
    for (stroke, mut transform) in &mut strokes {
        let center = root_rotation * (stroke.direction * LABEL_DISTANCE);
        let p0 = center + Vec3::new(stroke.start.x * LETTER_SIZE, stroke.start.y * LETTER_SIZE, 0.0);
        let p1 = center + Vec3::new(stroke.end.x * LETTER_SIZE, stroke.end.y * LETTER_SIZE, 0.0);
        *transform = transform_cylinder_between(p0, p1, LETTER_STROKE_RADIUS);
    }
}

fn transform_cylinder_between(p0: Vec3, p1: Vec3, radius: f32) -> Transform {
    let delta = p1 - p0;
    let length = delta.length();
    let midpoint = (p0 + p1) * 0.5;
    let direction = delta / length.max(f32::EPSILON);
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    Transform {
        translation: midpoint,
        rotation,
        scale: Vec3::new(radius, length * 0.5, radius),
    }
}

#[cfg(test)]
mod tests {
    use super::{structure_axes, widget_viewport};
    use bevy::prelude::{Color, UVec2, Vec3};

    #[test]
    fn widget_axes_use_structure_space_directions() {
        let directions = structure_axes();

        assert_eq!(directions[0].direction, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(directions[1].direction, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(directions[2].direction, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(directions[0].color, Color::srgb(1.0, 0.0, 0.0));
    }

    #[test]
    fn widget_viewport_anchors_to_lower_left() {
        let viewport = widget_viewport(800, 600, 144, 12, 12);

        assert_eq!(viewport.physical_position, UVec2::new(12, 444));
        assert_eq!(viewport.physical_size, UVec2::splat(144));
    }

    #[test]
    fn widget_viewport_clamps_large_offsets() {
        let viewport = widget_viewport(200, 150, 100, 180, 120);

        assert_eq!(viewport.physical_position, UVec2::new(180, 10));
        assert_eq!(viewport.physical_size, UVec2::splat(20));
    }
}
