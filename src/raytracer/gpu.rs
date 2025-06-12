use glam::{Mat4, UVec2, Vec3};
use wgpu::util::DeviceExt;

use crate::{common::{Camera, Frame, GpuContext, Scene, Texture}, raytracer::RaytracerImpl};

struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
    pub image_bind_group: wgpu::BindGroup,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
}

/// A gpu ray tracer
pub struct GpuRaytracer {
    render_target: Texture,
    compute_pipeline: ComputePipeline,
}

impl RaytracerImpl for GpuRaytracer {
    fn render(&self) {
        todo!()
    }
    
    fn output(&self) -> &crate::output::RaytracerOutput {
        todo!()
    }
}

impl GpuRaytracer {
    pub fn new(context: &GpuContext, scene: &Scene, size: UVec2) -> Self {
        let device = context.device();
        
        let render_target = Self::create_render_target(&device, size);

        let camera = scene.camera();
        let compute_pipeline = Self::create_compute_pipeline(&device, &render_target, camera);   

        GpuRaytracer {
            render_target,
            compute_pipeline,
        }
    }

    fn create_render_target(device: &wgpu::Device, size: UVec2) -> Texture {
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("render target texture"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("render target sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Texture { 
            texture, 
            view, 
            sampler
        }
    }

    fn create_compute_pipeline(device: &wgpu::Device, render_target: &Texture, camera: &Camera) -> ComputePipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("raytracer.wgsl").into()),
        });

        let image_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            }],
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<CameraData>() as u64),
                },
                count: None,
            }],
            label: Some("Camera bind group layout"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute pipeline layout"),
            bind_group_layouts: &[&image_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("cs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        let image_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image bind group"),
            layout: &image_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&render_target.view),
                },
            ],
        });

        let camera_data = CameraData::from_camera(camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("camera uniform buffer"),
                contents: bytemuck::bytes_of(&camera_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera bind group"),
        });

        ComputePipeline { 
            pipeline,
            image_bind_group,
            camera_bind_group,
            camera_buffer,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: UVec2) {
        // recreate render target
        self.render_target = Self::create_render_target(device, new_size);
    }

    /// Prepare rendering
    pub fn pre_render(&mut self, context: &GpuContext, scene: &mut Scene) {
        let camera = scene.camera_mut();
        if camera.is_dirty() {
            // Camera has changed, update gpu buffer
            let camera_data: CameraData = CameraData::from_camera(camera);
            context.queue().write_buffer(
                &self.compute_pipeline.camera_buffer, 
                0, 
                bytemuck::bytes_of(&camera_data)
            );
            camera.set_clean();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let mut compute_pass = frame.command_encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor {
                label: Some("compute pass"),
                timestamp_writes: None,
            }
        );

        compute_pass.set_pipeline(&self.compute_pipeline.pipeline);
        compute_pass.set_bind_group(0, &self.compute_pipeline.image_bind_group, &[]);
        compute_pass.set_bind_group(1, &self.compute_pipeline.camera_bind_group, &[]);
        compute_pass.dispatch_workgroups((frame.size.x + 7) / 8, (frame.size.y + 7) / 8, 1);

        drop(compute_pass);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraData {
    pub inv_view_proj: [[f32; 4]; 4],
    pub position: [f32; 3],
    pub _padding: f32,
}

impl Default for CameraData {
    fn default() -> Self {
        Self { 
            inv_view_proj: Mat4::IDENTITY.to_cols_array_2d(), 
            position: Vec3::ZERO.to_array(),
            _padding: 0.0,
        }
    }
}

impl CameraData {
    fn from_camera(camera: &Camera) -> CameraData {
        let view = Mat4::look_at_rh(
            camera.position(), 
            camera.target(), 
            camera.up()
        );
        let proj = Mat4::perspective_rh_gl(
            camera.field_of_view(), 
            camera.aspect_ratio(), 
            camera.near(), 
            camera.far()
        );
        let inv_view_proj = (proj * view).inverse();

        CameraData { 
            inv_view_proj: inv_view_proj.to_cols_array_2d(), 
            position: camera.position().to_array(), 
            _padding: 0.0 
        }
    }
}