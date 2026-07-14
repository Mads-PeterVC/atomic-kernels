mod capture;
mod config;
mod image_copy;

pub use config::{HeadlessRenderConfig, HeadlessRenderError};

use super::runtime::{asset_root, configure_shared_app};
use super::{ViewerCommand, ViewerConfig, ViewerSessionHandle};
use ak_core::{Structure, Trajectory};
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::transform::TransformSystems;
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCameraSystemSet};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread;
use std::time::Duration;

use capture::{
    CaptureResult, CaptureSettings, CaptureState, HeadlessTargetSpec, SessionDriverState,
    queue_capture, save_capture, setup_headless_target,
};
use image_copy::setup_image_copy;

pub fn export_image(
    trajectory: Trajectory,
    config: ViewerConfig,
    export: HeadlessRenderConfig,
) -> Result<(), HeadlessRenderError> {
    run_headless(trajectory, config, export, None, None)
}

pub fn export_structure_image(
    structure: Structure,
    config: ViewerConfig,
    export: HeadlessRenderConfig,
) -> Result<(), HeadlessRenderError> {
    export_image(Trajectory::new(vec![structure]), config, export)
}

pub fn export_image_with_session<F>(
    trajectory: Trajectory,
    config: ViewerConfig,
    export: HeadlessRenderConfig,
    driver: F,
) -> Result<(), HeadlessRenderError>
where
    F: FnOnce(ViewerSessionHandle) + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let handle = ViewerSessionHandle::new(sender);
    let driver_complete = Arc::new(AtomicBool::new(false));
    let driver_complete_thread = driver_complete.clone();
    let driver_thread = thread::Builder::new()
        .name("ak-headless-session-driver".to_string())
        .spawn(move || {
            driver(handle);
            driver_complete_thread.store(true, Ordering::Release);
        })
        .map_err(|err| HeadlessRenderError::new(format!("failed to spawn driver: {err}")))?;

    let result = run_headless(
        trajectory,
        config,
        export,
        Some(receiver),
        Some(driver_complete),
    );
    let _ = driver_thread.join();
    result
}

pub fn export_prepared_image(
    trajectory: Trajectory,
    config: ViewerConfig,
    export: HeadlessRenderConfig,
    receiver: mpsc::Receiver<ViewerCommand>,
) -> Result<(), HeadlessRenderError> {
    run_headless(trajectory, config, export, Some(receiver), None)
}

fn run_headless(
    trajectory: Trajectory,
    mut config: ViewerConfig,
    export: HeadlessRenderConfig,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    session_driver_complete: Option<Arc<AtomicBool>>,
) -> Result<(), HeadlessRenderError> {
    config.render.show_ui = false;
    config.render.show_orientation_widget = false;

    let result = CaptureResult {
        value: Arc::new(Mutex::new(None)),
    };
    let render_result = catch_unwind(AssertUnwindSafe(|| -> Result<(), HeadlessRenderError> {
        let mut app = build_app(
            trajectory,
            config,
            export,
            result.clone(),
            receiver,
            session_driver_complete,
        )?;
        app.run();
        result.take().unwrap_or_else(|| {
            Err(HeadlessRenderError::new(
                "headless render exited without producing a capture result",
            ))
        })
    }));

    match render_result {
        Ok(result) => result,
        Err(payload) => {
            let message = if let Some(message) = payload.downcast_ref::<String>() {
                message.clone()
            } else if let Some(message) = payload.downcast_ref::<&'static str>() {
                (*message).to_string()
            } else {
                "headless render panicked".to_string()
            };
            Err(HeadlessRenderError::new(message))
        }
    }
}

fn build_app(
    trajectory: Trajectory,
    config: ViewerConfig,
    export: HeadlessRenderConfig,
    result: CaptureResult,
    receiver: Option<mpsc::Receiver<ViewerCommand>>,
    session_driver_complete: Option<Arc<AtomicBool>>,
) -> Result<App, HeadlessRenderError> {
    if export.width == 0 || export.height == 0 {
        return Err(HeadlessRenderError::new(
            "headless render image size must be non-zero",
        ));
    }

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<bevy::log::LogPlugin>()
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: asset_root(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .disable::<WinitPlugin>(),
    )
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));

    app.insert_resource(HeadlessTargetSpec {
        width: export.width,
        height: export.height,
    })
    .insert_resource(CaptureSettings {
        path: export.path,
        stable_frames: export.stable_frames.max(1),
    })
    .insert_resource(CaptureState {
        preroll_remaining: export.preroll_frames,
        stable_frames: 0,
        requested: false,
        request_delay_remaining: 0,
    })
    .insert_resource(result);

    if let Some(complete) = session_driver_complete {
        app.insert_resource(SessionDriverState { complete });
    }

    configure_shared_app(
        &mut app,
        trajectory,
        config,
        receiver,
        Arc::new(Mutex::new(super::session::ViewerSnapshot::default())),
    );
    app.add_systems(
        Startup,
        setup_headless_target.before(super::systems::setup_camera),
    );
    app.add_systems(
        PostUpdate,
        queue_capture
            .after(PanOrbitCameraSystemSet)
            .after(TransformSystems::Propagate),
    );
    setup_image_copy(&mut app);
    app.add_systems(PostUpdate, save_capture.after(queue_capture));

    Ok(app)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BondList, Face, FaceList};
    use image::GenericImageView;
    use std::path::{Path, PathBuf};
    use std::sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    };

    static UNIQUE_ID: AtomicU64 = AtomicU64::new(0);
    static HEADLESS_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn fixture_structure() -> Structure {
        Structure::new(
            vec![
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.74],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            vec![8, 1, 1, 1],
            [[8.0, 0.0, 0.0], [0.0, 8.0, 0.0], [0.0, 0.0, 8.0]],
            [false, false, false],
        )
    }

    fn temp_png(name: &str) -> PathBuf {
        let id = UNIQUE_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("ak-headless-{name}-{id}.png"))
    }

    fn test_render_config(path: &Path) -> HeadlessRenderConfig {
        let mut config = HeadlessRenderConfig::new(path, 320, 240);
        if std::env::var_os("CI").is_some() {
            config.preroll_frames = 48;
            config.stable_frames = 12;
        }
        config
    }

    fn should_run_headless_render_tests() -> bool {
        if std::env::var_os("ATOMIC_KERNELS_RUN_RUST_HEADLESS_TESTS").is_some() {
            return true;
        }
        std::env::var_os("CI").is_none()
    }

    fn image_has_content(path: &Path, width: u32, height: u32) -> bool {
        let image = image::open(path).expect("saved image should be readable");
        assert_eq!(image.dimensions(), (width, height));
        let rgba = image.to_rgba8();
        let mut unique = std::collections::BTreeSet::new();
        for pixel in rgba.pixels() {
            unique.insert(pixel.0);
            if unique.len() > 8 {
                break;
            }
        }
        unique.len() > 1
    }

    fn assert_render_succeeds<F>(name: &str, mut render_once: F)
    where
        F: FnMut(&Path) -> Result<(), HeadlessRenderError>,
    {
        let attempts = if std::env::var_os("CI").is_some() {
            5
        } else {
            1
        };
        for attempt in 0..attempts {
            let path = temp_png(name);
            match render_once(&path) {
                Ok(()) => {
                    if image_has_content(&path, 320, 240) {
                        let _ = std::fs::remove_file(path);
                        return;
                    }
                }
                Err(err) if err.to_string().contains("Unable to find a GPU") => return,
                Err(err) => panic!("headless export should succeed: {err}"),
            }
            let _ = std::fs::remove_file(&path);
            if attempt + 1 == attempts {
                panic!("rendered image should not be a flat color");
            }
        }
    }

    #[test]
    fn exports_default_scene_to_png() {
        if !should_run_headless_render_tests() {
            return;
        }
        let _guard = HEADLESS_TEST_LOCK.lock().unwrap();
        let config = ViewerConfig::default();
        assert_render_succeeds("default", |path| {
            export_structure_image(
                fixture_structure(),
                config.clone(),
                test_render_config(path),
            )
        });
    }

    #[test]
    fn exports_scripted_scene_with_shared_session_commands() {
        if !should_run_headless_render_tests() {
            return;
        }
        let _guard = HEADLESS_TEST_LOCK.lock().unwrap();
        let mut config = ViewerConfig::default();
        config.render.show_axes = false;
        assert_render_succeeds("scripted", |path| {
            export_image_with_session(
                Trajectory::new(vec![fixture_structure()]),
                config.clone(),
                test_render_config(path),
                |session| {
                    session
                        .set_bonds(BondList::new([(0, 1), (0, 2), (0, 3)]), Some(0))
                        .unwrap();
                    session
                        .set_faces(
                            FaceList::new([Face::new([1, 2, 3], [0.2, 0.6, 0.9, 0.35]).unwrap()]),
                            Some(0),
                        )
                        .unwrap();
                    session
                        .set_render_style(
                            super::super::RenderStyle::BallAndStick(
                                super::super::BallAndStickStyle {
                                    atom_scale: 0.5,
                                    bond_radius: 0.06,
                                    bond_color: [0.5, 0.5, 0.5, 1.0],
                                    bond_scope: super::super::BondScope::TouchSelection,
                                },
                            ),
                            vec![true, true, true, true],
                            Some(0),
                            false,
                        )
                        .unwrap();
                    session.frame_all().unwrap();
                },
            )
        });
    }
}
