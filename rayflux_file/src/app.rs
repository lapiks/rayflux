use std::{path::Path, time::Instant};

use glam::UVec2;
use image::{ImageBuffer, Rgba};

use rayflux::{
    common::{GpuContext, Object, Scene, Texture}, 
    raytracer::{cpu::CpuRaytracer, gpu::GpuRaytracer, RaytracerType}
};

pub struct App {
    raytracer_type: RaytracerType,
}

impl App {
    pub fn new(raytracer_type: RaytracerType) -> Self {
        Self { raytracer_type }
    }

    pub fn run(&mut self) {
        let default_size = UVec2::new(1800, 900);

        // Create a scene
        let mut scene = Scene::default();
        scene.add_object(Object::new_sphere());

        // Prepare camera
        let camera = scene.camera_mut();
        camera.update_aspect_ratio(default_size);

        let now = Instant::now();

        println!("Start {} rendering", self.raytracer_type);
        
        match self.raytracer_type {
            RaytracerType::Cpu => {
                // Create raytracer
                let mut raytracer = CpuRaytracer::new(default_size);
                // Execute raytracer
                raytracer.render(&scene);
                // save result as image
                let canvas = raytracer.canvas();
                let _ = canvas.export("output/cpu/test.png");
            },
            RaytracerType::Gpu => {
                // Create gpu context
                let context = pollster::block_on(GpuContext::new());

                // Create raytracer
                let mut raytracer = GpuRaytracer::new(&context, &scene, default_size);

                // Create a command encoder
                let mut command_encoder = context.device().create_command_encoder(&Default::default());

                // Execute raytracer
                raytracer.render(&mut command_encoder);

                // Submit command encoder 
                context.queue().submit(Some(command_encoder.finish()));

                // save result as image
                let output = raytracer.render_target();
                pollster::block_on(save_texture_as_png(&context, output, "output/gpu/test.png"));
            } 
        };

        println!("Rendering finished in {:.2?} seconds", now.elapsed());
    }
}

async fn save_texture_as_png<P: AsRef<Path>>(
    context: &GpuContext,
    texture: &Texture,
    path: P,
) {
    let device = context.device();

    let bytes_per_pixel = 4; // RGBA8
    let padded_bytes_per_row = (texture.size.x * bytes_per_pixel + 255) & !255;
    let unpadded_bytes_per_row = texture.size.x * bytes_per_pixel;
    let buffer_size = padded_bytes_per_row * texture.size.y;

    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Texture Readback Buffer"),
        size: buffer_size as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    command_encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &output_staging_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(texture.size.y),
            },
        },
        wgpu::Extent3d {
            width: texture.size.x,
            height: texture.size.y,
            depth_or_array_layers: 1,
        },
    );

    context.queue().submit(Some(command_encoder.finish()));

    // Wait for buffer
    let buffer_slice = output_staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
    device.poll(wgpu::Maintain::Wait);
    receiver.recv_async().await.unwrap().unwrap();

    // Read datas
    let mut pixels_data = Vec::with_capacity((texture.size.x * texture.size.y * bytes_per_pixel) as usize);
    {
        let view = buffer_slice.get_mapped_range();

        for y in 0..texture.size.y {
            let row_start = y * padded_bytes_per_row;
            let row_end = row_start + unpadded_bytes_per_row;
            let row = &view[row_start as usize..row_end as usize];

            for chunk in row.chunks_exact(4) {
                // Gamma correction
                let r = linear_to_srgb(chunk[0]);
                let g = linear_to_srgb(chunk[1]);
                let b = linear_to_srgb(chunk[2]);
                let a = chunk[3]; // alpha untouched
                pixels_data.extend_from_slice(&[r, g, b, a]);
            }
        }
    }

    output_staging_buffer.unmap();

    let image: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(texture.size.x, texture.size.y, pixels_data).unwrap();

    image
        .save_with_format(path, image::ImageFormat::Png)
        .expect("Failed to save PNG file");
}

/// Gamma correction
fn linear_to_srgb(x: u8) -> u8 {
    let f = x as f32 / 255.0;
    ((f.powf(1.0 / 2.2)) * 255.0).round().clamp(0.0, 255.0) as u8
}