use super::runtime::{MainCameraRenderTarget, asset_root, configure_shared_app};
use super::{ViewerCommand, ViewerConfig, ViewerSessionHandle};
use ak_core::{Structure, Trajectory};
use bevy::app::{AppExit, ScheduleRunnerPlugin};
use bevy::camera::RenderTarget;
use bevy::image::TextureFormatPixelInfo;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{self, NodeRunError, RenderGraph, RenderGraphContext};
use bevy::render::render_resource::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode, PollType,
    TexelCopyBufferInfo, TexelCopyBufferLayout, TextureFormat, TextureUsages,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy::transform::TransformSystems;
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCameraSystemSet};
use crossbeam_channel::{Receiver, Sender};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread;
use std::time::Duration;

const DEFAULT_PREROLL_FRAMES: u32 = 4;
const DEFAULT_STABLE_FRAMES: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadlessRenderConfig {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub preroll_frames: u32,
    pub stable_frames: u32,
}

impl HeadlessRenderConfig {
    pub fn new(path: impl Into<PathBuf>, width: u32, height: u32) -> Self {
        Self {
            path: path.into(),
            width,
            height,
            preroll_frames: DEFAULT_PREROLL_FRAMES,
            stable_frames: DEFAULT_STABLE_FRAMES,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadlessRenderError {
    message: String,
}

impl HeadlessRenderError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for HeadlessRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for HeadlessRenderError {}

impl From<std::io::Error> for HeadlessRenderError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

#[derive(Resource, Clone)]
struct CaptureSettings {
    path: PathBuf,
    stable_frames: u32,
}

#[derive(Resource, Clone, Copy)]
struct HeadlessTargetSpec {
    width: u32,
    height: u32,
}

#[derive(Resource, Default)]
struct CaptureState {
    preroll_remaining: u32,
    stable_frames: u32,
    requested: bool,
    request_delay_remaining: u32,
}

#[derive(Resource, Clone)]
struct CaptureResult {
    value: Arc<Mutex<Option<Result<(), HeadlessRenderError>>>>,
}

impl CaptureResult {
    fn store(&self, result: Result<(), HeadlessRenderError>) {
        if let Ok(mut slot) = self.value.lock() {
            *slot = Some(result);
        }
    }

    fn take(&self) -> Option<Result<(), HeadlessRenderError>> {
        self.value.lock().ok().and_then(|mut slot| slot.take())
    }
}

#[derive(Resource, Deref)]
struct MainWorldReceiver(Receiver<Vec<u8>>);

#[derive(Resource, Clone)]
struct SessionDriverState {
    complete: Arc<AtomicBool>,
}

#[derive(Resource, Deref)]
struct RenderWorldSender(Sender<Vec<u8>>);

#[derive(Component, Deref, DerefMut)]
struct ImageToSave(Handle<Image>);

#[derive(Clone, Default, Resource, Deref, DerefMut)]
struct ImageCopiers(pub Vec<ImageCopier>);

#[derive(Clone, Component)]
struct ImageCopier {
    buffer: Buffer,
    src_image: Handle<Image>,
}

#[derive(bevy::render::render_graph::RenderLabel, Debug, PartialEq, Eq, Clone, Hash)]
struct ImageCopy;

#[derive(Default)]
struct ImageCopyDriver;

impl ImageCopier {
    fn new(src_image: Handle<Image>, size: Extent3d, render_device: &RenderDevice) -> Self {
        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(size.width as usize * 4);
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: padded_bytes_per_row as u64 * size.height as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { buffer, src_image }
    }
}

impl render_graph::Node for ImageCopyDriver {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let image_copiers = world
            .get_resource::<ImageCopiers>()
            .expect("image copiers should be extracted");
        let gpu_images = world
            .get_resource::<RenderAssets<bevy::render::texture::GpuImage>>()
            .expect("gpu images should exist");

        for image_copier in image_copiers.iter() {
            let src_image = gpu_images
                .get(&image_copier.src_image)
                .expect("render target image should exist");
            let mut encoder = render_context
                .render_device()
                .create_command_encoder(&CommandEncoderDescriptor::default());

            let block_dimensions = src_image.texture_format.block_dimensions();
            let block_size = src_image.texture_format.block_copy_size(None).unwrap();
            let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
                (src_image.size.width as usize / block_dimensions.0 as usize) * block_size as usize,
            );

            encoder.copy_texture_to_buffer(
                src_image.texture.as_image_copy(),
                TexelCopyBufferInfo {
                    buffer: &image_copier.buffer,
                    layout: TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(padded_bytes_per_row as u32),
                        rows_per_image: None,
                    },
                },
                src_image.size,
            );

            let render_queue = world.get_resource::<RenderQueue>().unwrap();
            render_queue.submit(std::iter::once(encoder.finish()));
        }

        Ok(())
    }
}

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

    configure_shared_app(&mut app, trajectory, config, receiver);
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

fn setup_headless_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
    spec: Res<HeadlessTargetSpec>,
) {
    let size = Extent3d {
        width: spec.width,
        height: spec.height,
        ..default()
    };
    let mut render_target_image =
        Image::new_target_texture(size.width, size.height, TextureFormat::bevy_default(), None);
    render_target_image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    let render_target_image_handle = images.add(render_target_image);

    let cpu_image =
        Image::new_target_texture(size.width, size.height, TextureFormat::bevy_default(), None);
    let cpu_image_handle = images.add(cpu_image);

    commands.spawn(ImageCopier::new(
        render_target_image_handle.clone(),
        size,
        &render_device,
    ));
    commands.spawn(ImageToSave(cpu_image_handle));
    commands.insert_resource(MainCameraRenderTarget(RenderTarget::Image(
        render_target_image_handle.into(),
    )));
}

fn setup_image_copy(app: &mut App) {
    let (sender, receiver) = crossbeam_channel::unbounded();
    app.insert_resource(MainWorldReceiver(receiver));

    let render_app = app.sub_app_mut(RenderApp);
    let mut graph = render_app.world_mut().resource_mut::<RenderGraph>();
    graph.add_node(ImageCopy, ImageCopyDriver);
    graph.add_node_edge(bevy::render::graph::CameraDriverLabel, ImageCopy);

    render_app
        .insert_resource(RenderWorldSender(sender))
        .add_systems(ExtractSchedule, image_copy_extract)
        .add_systems(
            Render,
            receive_image_from_buffer.after(RenderSystems::Render),
        );
}

fn image_copy_extract(mut commands: Commands, image_copiers: Extract<Query<&ImageCopier>>) {
    commands.insert_resource(ImageCopiers(image_copiers.iter().cloned().collect()));
}

fn receive_image_from_buffer(
    image_copiers: Res<ImageCopiers>,
    render_device: Res<RenderDevice>,
    sender: Res<RenderWorldSender>,
) {
    for image_copier in image_copiers.iter() {
        let buffer_slice = image_copier.buffer.slice(..);
        let (buffer_sender, buffer_receiver) = crossbeam_channel::bounded(1);

        buffer_slice.map_async(MapMode::Read, move |result| {
            let _ = buffer_sender.send(result);
        });

        render_device
            .poll(PollType::wait_indefinitely())
            .expect("failed to poll render device");
        buffer_receiver
            .recv()
            .expect("failed to receive map_async result")
            .expect("failed to map image buffer");

        let _ = sender.send(buffer_slice.get_mapped_range().to_vec());
        image_copier.buffer.unmap();
    }
}

fn queue_capture(
    mut state: ResMut<CaptureState>,
    settings: Res<CaptureSettings>,
    viewer: Res<super::ViewerState>,
    camera: Res<super::CameraState>,
    session_driver_state: Option<Res<SessionDriverState>>,
) {
    if state.requested {
        return;
    }

    if let Some(session_driver_state) = session_driver_state
        && !session_driver_state.complete.load(Ordering::Acquire)
    {
        state.stable_frames = 0;
        return;
    }

    if viewer.needs_render || camera.needs_apply {
        state.stable_frames = 0;
        return;
    }

    if state.preroll_remaining > 0 {
        state.preroll_remaining -= 1;
        return;
    }

    state.stable_frames += 1;
    if state.stable_frames >= settings.stable_frames {
        state.requested = true;
        state.request_delay_remaining = 1;
    }
}

fn save_capture(
    images_to_save: Query<&ImageToSave>,
    receiver: Res<MainWorldReceiver>,
    mut images: ResMut<Assets<Image>>,
    settings: Res<CaptureSettings>,
    mut state: ResMut<CaptureState>,
    result: Res<CaptureResult>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if !state.requested {
        return;
    }
    if state.request_delay_remaining > 0 {
        state.request_delay_remaining -= 1;
        return;
    }

    let mut image_data = Vec::new();
    while let Ok(data) = receiver.try_recv() {
        image_data = data;
    }
    if image_data.is_empty() {
        result.store(Err(HeadlessRenderError::new(
            "headless render did not receive any image data",
        )));
        app_exit.write(AppExit::Success);
        return;
    }

    let save_result = save_image_data(&settings.path, images_to_save, &mut images, &image_data);
    result.store(save_result.map_err(HeadlessRenderError::from));
    state.requested = false;
    app_exit.write(AppExit::Success);
}

fn save_image_data(
    path: &Path,
    images_to_save: Query<&ImageToSave>,
    images: &mut Assets<Image>,
    image_data: &[u8],
) -> std::io::Result<()> {
    let Some(image) = images_to_save.iter().next() else {
        return Err(std::io::Error::other("missing CPU image target"));
    };
    let img_bytes = images
        .get_mut(image.id())
        .ok_or_else(|| std::io::Error::other("missing saved image"))?;
    let row_bytes =
        img_bytes.width() as usize * img_bytes.texture_descriptor.format.pixel_size().unwrap();
    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
    if row_bytes == aligned_row_bytes {
        img_bytes
            .data
            .as_mut()
            .unwrap()
            .clone_from_slice(image_data);
    } else {
        img_bytes.data = Some(
            image_data
                .chunks(aligned_row_bytes)
                .take(img_bytes.height() as usize)
                .flat_map(|row| &row[..row_bytes.min(row.len())])
                .copied()
                .collect(),
        );
    }

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let image = img_bytes
        .clone()
        .try_into_dynamic()
        .map_err(|err| std::io::Error::other(err.to_string()))?;
    image
        .to_rgba8()
        .save(path)
        .map_err(|err| std::io::Error::other(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BondList, Face, FaceList};
    use image::GenericImageView;
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
            config.preroll_frames = 24;
            config.stable_frames = 8;
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
        let attempts = if std::env::var_os("CI").is_some() { 3 } else { 1 };
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
            export_structure_image(fixture_structure(), config.clone(), test_render_config(path))
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
            export_image_with_session(Trajectory::new(vec![fixture_structure()]), config.clone(), test_render_config(path), |session| {
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
                        super::super::RenderStyle::BallAndStick(super::super::BallAndStickStyle {
                            atom_scale: 0.5,
                            bond_radius: 0.06,
                            bond_color: [0.5, 0.5, 0.5, 1.0],
                            bond_scope: super::super::BondScope::TouchSelection,
                        }),
                        vec![true, true, true, true],
                        Some(0),
                        false,
                    )
                    .unwrap();
                session.frame_all().unwrap();
            })
        });
    }
}
