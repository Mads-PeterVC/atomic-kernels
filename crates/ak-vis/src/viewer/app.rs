use ak_core::{Structure, Trajectory};

use crate::viewer::ViewerConfig;
use crate::viewer::systems::*;

use crate::viewer::controls::{
    keyboard_controls, screenshot_on_spacebar, screenshot_saving, toggle_view, navigate_frames
};

use crate::ui::setup_ui;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

#[derive(Resource)]
pub struct ViewerTrajectory {
    pub traj: Trajectory,
    pub current: usize,
}

pub fn run(trajectory: Trajectory, config: ViewerConfig) {
    let mut app = App::new();
    app.insert_resource(ClearColor(config.color.background))
        .insert_resource(ViewerTrajectory {
            traj: trajectory,
            current: config.initial_frame,
        })
        .insert_resource(config)
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(
            Startup,
            (
                setup_lighting,
                setup_camera,
                render_current_frame,
                setup_camera_light,
            ),
        )
        .add_systems(
            Update,
            (
                toggle_view,
                keyboard_controls,
                update_camera_light,
                screenshot_on_spacebar,
                screenshot_saving,
                navigate_frames,
            ),
        );
    if app.world().resource::<ViewerConfig>().render.show_ui {
        app.add_systems(Startup, setup_ui);
    }
    app.run();
}

pub fn run_default(trajectory: Trajectory) {
    let config = ViewerConfig::default();
    run(trajectory, config)
}

pub fn run_structure(structure: Structure, config: ViewerConfig) {
    let trajectory = Trajectory::new(vec![structure]);
    run(trajectory, config)
}

pub fn run_structure_default(structure: Structure) {
    let trajectory = Trajectory::new(vec![structure]);
    run_default(trajectory);
}
