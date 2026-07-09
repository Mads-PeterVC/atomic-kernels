use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{self, NodeRunError, RenderGraph, RenderGraphContext};
use bevy::render::render_resource::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode, PollType,
    TexelCopyBufferInfo, TexelCopyBufferLayout,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use crossbeam_channel::{Receiver, Sender};

#[derive(Resource, Deref)]
pub(super) struct MainWorldReceiver(Receiver<Vec<u8>>);

#[derive(Resource, Deref)]
struct RenderWorldSender(Sender<Vec<u8>>);

#[derive(Component, Deref, DerefMut)]
pub(super) struct ImageToSave(pub(super) Handle<Image>);

#[derive(Clone, Default, Resource, Deref, DerefMut)]
pub(super) struct ImageCopiers(pub Vec<ImageCopier>);

#[derive(Clone, Component)]
pub(super) struct ImageCopier {
    buffer: Buffer,
    src_image: Handle<Image>,
}

#[derive(bevy::render::render_graph::RenderLabel, Debug, PartialEq, Eq, Clone, Hash)]
struct ImageCopy;

#[derive(Default)]
struct ImageCopyDriver;

impl ImageCopier {
    pub(super) fn new(
        src_image: Handle<Image>,
        size: Extent3d,
        render_device: &RenderDevice,
    ) -> Self {
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

pub(super) fn setup_image_copy(app: &mut App) {
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
