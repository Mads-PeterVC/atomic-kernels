use crate::ui::{InspectorState, PlaybackState};
use crate::viewer::ViewerConfig;
use crate::viewer::session::{CameraState, ViewerCommand, ViewerSnapshot, ViewerState};
use crate::viewer::systems::{
    advance_camera_motion, apply_camera_state, apply_viewer_commands, render_current_frame,
    rerender_if_dirty, setup_camera, setup_camera_light, setup_lighting, sync_viewer_snapshot,
    update_camera_light,
};
use ak_core::Trajectory;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::{Arc, Mutex, mpsc};

const ROBOTO_MONO_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/RobotoMono-VariableFont_wght.ttf"
));
const NOTO_SYMBOLS_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/NotoSansSymbols2-Regular.ttf"
));

#[derive(Resource)]
pub(crate) struct CommandReceiver(pub Option<Mutex<mpsc::Receiver<ViewerCommand>>>);

#[derive(Resource, Clone)]
pub(crate) struct MainCameraRenderTarget(pub RenderTarget);

#[derive(Resource, Clone)]
pub(crate) struct SharedViewerSnapshot(pub Arc<Mutex<ViewerSnapshot>>);

#[derive(Resource, Clone)]
pub(crate) struct ViewerFonts {
    pub(crate) mono: Handle<Font>,
    pub(crate) symbols: Handle<Font>,
}

pub(crate) fn register_viewer_fonts(app: &mut App) {
    let mut fonts = app.world_mut().resource_mut::<Assets<Font>>();
    let mono = fonts.add(
        Font::try_from_bytes(ROBOTO_MONO_FONT.to_vec())
            .expect("bundled Roboto Mono font must be a valid font asset"),
    );
    let symbols = fonts.add(
        Font::try_from_bytes(NOTO_SYMBOLS_FONT.to_vec())
            .expect("bundled Noto Symbols font must be a valid font asset"),
    );
    app.insert_resource(ViewerFonts { mono, symbols });
}

pub(crate) fn asset_root() -> String {
    if let Some(path) = std::env::var_os("AK_VIS_ASSET_ROOT") {
        return path.to_string_lossy().into_owned();
    }

    bundled_asset_root().to_string_lossy().into_owned()
}

fn bundled_asset_root() -> &'static Path {
    static ASSET_ROOT: OnceLock<PathBuf> = OnceLock::new();

    ASSET_ROOT
        .get_or_init(|| {
            let root = std::env::temp_dir()
                .join("atomic-kernels")
                .join("ak-vis-assets")
                .join(env!("CARGO_PKG_VERSION"));
            let fonts_dir = root.join("fonts");
            fs::create_dir_all(&fonts_dir).expect("failed to create bundled asset directory");
            write_bundled_asset(
                &fonts_dir.join("RobotoMono-VariableFont_wght.ttf"),
                ROBOTO_MONO_FONT,
            );
            write_bundled_asset(
                &fonts_dir.join("NotoSansSymbols2-Regular.ttf"),
                NOTO_SYMBOLS_FONT,
            );
            root
        })
        .as_path()
}

fn write_bundled_asset(path: &Path, bytes: &[u8]) {
    let needs_write = match fs::metadata(path) {
        Ok(metadata) => metadata.len() != bytes.len() as u64,
        Err(_) => true,
    };

    if needs_write {
        fs::write(path, bytes).unwrap_or_else(|err| {
            panic!("failed to write bundled asset {}: {err}", path.display())
        });
    }
}

pub(crate) fn configure_shared_app(
    app: &mut App,
    trajectory: Trajectory,
    config: ViewerConfig,
    mut receiver: Option<mpsc::Receiver<ViewerCommand>>,
    snapshot: Arc<Mutex<ViewerSnapshot>>,
) {
    let mut viewer_state = ViewerState::new(trajectory, config.initial_frame);
    viewer_state.supercell.repeats = [
        config.render.supercell_repeat_a,
        config.render.supercell_repeat_b,
        config.render.supercell_repeat_c,
    ];
    viewer_state.supercell.ghost_repeated_images = config.render.ghost_repeated_images;
    let mut camera_state = CameraState::new(&viewer_state);
    let mut has_startup_camera_command = false;

    if let Some(receiver_ref) = receiver.as_mut() {
        while let Ok(command) = receiver_ref.try_recv() {
            has_startup_camera_command |= is_camera_command(&command);
            camera_state.apply_command(&viewer_state, &command);
            let _ = viewer_state.apply_command(command);
        }
    }

    if has_startup_camera_command {
        viewer_state.needs_camera_reset = false;
    }

    app.insert_resource(ClearColor(config.color.background))
        .insert_resource(viewer_state)
        .insert_resource(camera_state)
        .insert_resource(config)
        .insert_resource(InspectorState::default())
        .insert_resource(PlaybackState::default())
        .insert_resource(SharedViewerSnapshot(snapshot))
        .insert_resource(CommandReceiver(receiver.map(Mutex::new)))
        .add_systems(
            Startup,
            (
                setup_lighting,
                setup_camera,
                render_current_frame,
                setup_camera_light,
                sync_viewer_snapshot,
            ),
        )
        .add_systems(
            Update,
            (
                apply_viewer_commands,
                advance_camera_motion,
                apply_camera_state,
                update_camera_light,
                rerender_if_dirty,
                sync_viewer_snapshot,
            ),
        );
}

fn is_camera_command(command: &ViewerCommand) -> bool {
    matches!(
        command,
        ViewerCommand::SetCameraView { .. }
            | ViewerCommand::PanCamera { .. }
            | ViewerCommand::ZoomCamera { .. }
            | ViewerCommand::OrbitCamera { .. }
            | ViewerCommand::FrameAll
            | ViewerCommand::StartOrbit { .. }
            | ViewerCommand::StopCameraMotion
    )
}

#[cfg(test)]
mod tests {
    use super::{bundled_asset_root, configure_shared_app};
    use crate::viewer::ViewerConfig;
    use crate::viewer::session::{CameraState, ViewerCommand, ViewerSnapshot, ViewerState};
    use ak_core::{Structure, Trajectory};
    use bevy::prelude::App;
    use std::sync::{Arc, Mutex, mpsc};

    #[test]
    fn bundled_asset_root_contains_expected_fonts() {
        let root = bundled_asset_root();
        assert!(
            root.join("fonts/RobotoMono-VariableFont_wght.ttf")
                .is_file()
        );
        assert!(root.join("fonts/NotoSansSymbols2-Regular.ttf").is_file());
    }

    fn structure() -> Structure {
        Structure::new(
            vec![[0.0, 0.0, 0.0]],
            vec![1],
            [[6.0, 0.0, 0.0], [0.0, 6.0, 0.0], [0.0, 0.0, 6.0]],
            [false; 3],
        )
    }

    #[test]
    fn startup_camera_commands_skip_default_camera_reset() {
        let (sender, receiver) = mpsc::channel();
        sender
            .send(ViewerCommand::SetCameraView {
                focus: Some([1.0, 2.0, 3.0]),
                radius: Some(4.0),
                yaw: Some(0.7),
                pitch: Some(0.2),
            })
            .unwrap();

        let mut app = App::new();
        configure_shared_app(
            &mut app,
            Trajectory::new(vec![structure()]),
            ViewerConfig::default(),
            Some(receiver),
            Arc::new(Mutex::new(ViewerSnapshot::default())),
        );

        let viewer = app.world().resource::<ViewerState>();
        let camera = app.world().resource::<CameraState>();

        assert!(!viewer.needs_camera_reset);
        assert_eq!(camera.focus.to_array(), [1.0, 2.0, 3.0]);
        assert_eq!(camera.radius, 4.0);
        assert_eq!(camera.yaw, 0.7);
        assert_eq!(camera.pitch, 0.2);
    }
}
