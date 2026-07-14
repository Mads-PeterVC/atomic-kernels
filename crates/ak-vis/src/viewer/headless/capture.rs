use super::config::HeadlessRenderError;
use super::image_copy::{ImageCopier, ImageToSave, MainWorldReceiver};
use crate::viewer::runtime::MainCameraRenderTarget;
use crate::viewer::{CameraState, ViewerState};
use bevy::app::AppExit;
use bevy::camera::RenderTarget;
use bevy::image::TextureFormatPixelInfo;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureFormat, TextureUsages};
use bevy::render::renderer::RenderDevice;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

#[derive(Resource, Clone)]
pub(super) struct CaptureSettings {
    pub(super) path: PathBuf,
    pub(super) stable_frames: u32,
}

#[derive(Resource, Clone, Copy)]
pub(super) struct HeadlessTargetSpec {
    pub(super) width: u32,
    pub(super) height: u32,
}

#[derive(Resource, Default)]
pub(super) struct CaptureState {
    pub(super) preroll_remaining: u32,
    pub(super) stable_frames: u32,
    pub(super) requested: bool,
    pub(super) request_delay_remaining: u32,
}

#[derive(Resource, Clone)]
pub(super) struct CaptureResult {
    pub(super) value: Arc<Mutex<Option<Result<(), HeadlessRenderError>>>>,
}

impl CaptureResult {
    fn store(&self, result: Result<(), HeadlessRenderError>) {
        if let Ok(mut slot) = self.value.lock() {
            *slot = Some(result);
        }
    }

    pub(super) fn take(&self) -> Option<Result<(), HeadlessRenderError>> {
        self.value.lock().ok().and_then(|mut slot| slot.take())
    }
}

#[derive(Resource, Clone)]
pub(super) struct SessionDriverState {
    pub(super) complete: Arc<AtomicBool>,
}

pub(super) fn setup_headless_target(
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

pub(super) fn queue_capture(
    mut state: ResMut<CaptureState>,
    settings: Res<CaptureSettings>,
    viewer: Res<ViewerState>,
    camera: Res<CameraState>,
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

pub(super) fn save_capture(
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
