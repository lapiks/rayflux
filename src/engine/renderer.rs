use std::sync::Arc;

use glam::UVec2;
use winit::window::Window;

use crate::engine::{CameraUniform, World};

pub struct Frame {
    pub surface_texture: wgpu::SurfaceTexture,
    pub surface_view: wgpu::TextureView,
    pub command_encoder: wgpu::CommandEncoder,
}

struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

struct RenderPipeline {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline,
}

struct ComputePipeline {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline: wgpu::ComputePipeline,
}

/// wgpu renderer
pub struct Renderer {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    size: UVec2,
    render_target: Texture,
    render_pipeline: RenderPipeline,
    render_bind_group: wgpu::BindGroup,
    compute_pipeline: ComputePipeline,
    compute_bind_group: wgpu::BindGroup,
    camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let size = UVec2 { x: size.width, y: size.height };

        let render_target = Self::create_render_texture(&device, size);

        let render_pipeline = Self::create_render_pipeline(&device, surface_format);
        let render_bind_group = Self::create_render_bind_group(&device, &render_pipeline.bind_group_layout, &render_target);

        let camera_bind_group_layout = Self::create_camera_bind_group_layout(&device);

        let compute_pipeline = Self::create_compute_pipeline(&device, &camera_bind_group_layout);
        let compute_bind_group = Self::create_compute_bind_group(&device, &compute_pipeline.bind_group_layout, &render_target);        

        let renderer = Renderer {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            render_pipeline,
            render_target,
            render_bind_group,
            compute_pipeline,
            compute_bind_group,
            camera_bind_group_layout,
        };

        // Configure surface for the first time
        renderer.configure_surface();

        renderer
    }

    fn create_render_texture(device: &wgpu::Device, size: UVec2) -> Texture {
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

    fn create_render_pipeline(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/render.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("render pipeline bind group layout"),
            entries: &[
                // Binding 0 : texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Binding 1 : sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], 
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { 
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { 
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        RenderPipeline {
            bind_group_layout,
            pipeline,
        }
    }

    fn create_render_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        render_target: &Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("render pipeline bind group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&render_target.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&render_target.sampler),
                },
            ],
        })
    }

    fn create_compute_pipeline(device: &wgpu::Device, camera_bind_group_layout: &wgpu::BindGroupLayout) -> ComputePipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/compute.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute bind group layout"),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute pipeline layout"),
            bind_group_layouts: &[&bind_group_layout, camera_bind_group_layout],
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

        ComputePipeline { 
            bind_group_layout, 
            pipeline 
        }
    }

    fn create_compute_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout, 
        render_target: &Texture
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute bind group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&render_target.view),
                },
            ],
        })
    }

    fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<CameraUniform>() as u64),
                },
                count: None,
            }],
            label: Some("Camera Bind Group Layout"),
        })
    }

    pub fn camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.camera_bind_group_layout
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_format
    }

    pub fn resize(&mut self, new_size: UVec2) {
        self.size = new_size;

        // recreate render target
        self.render_target = Self::create_render_texture(&self.device, new_size);

        // reconfigure the surface
        self.configure_surface();
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.x,
            height: self.size.y,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn begin_frame(&self) -> Result<Frame, wgpu::SurfaceError>{
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        // Create a command encoder
        let command_encoder = self.device.create_command_encoder(&Default::default());

        Ok(Frame {
            surface_texture,
            surface_view,
            command_encoder,
        })
    }

    pub fn end_frame(&self, frame: Frame) {
        // Submit the command in the queue to execute
        self.queue.submit([frame.command_encoder.finish()]);
        self.window.pre_present_notify();
        frame.surface_texture.present();
    }

    pub fn render(&mut self, frame: &mut Frame, world: &World) {
        let mut compute_pass = frame.command_encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor {
                label: Some("compute pass"),
                timestamp_writes: None,
            }
        );

        compute_pass.set_pipeline(&self.compute_pipeline.pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);

        let camera_bind_group = world.camera().bind_group().expect("missing bind group");
        compute_pass.set_bind_group(1, camera_bind_group, &[]);
        compute_pass.dispatch_workgroups((self.size.x + 7) / 8, (self.size.y + 7) / 8, 1);

        drop(compute_pass);

        let mut render_pass = frame.command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &frame.surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(
                        wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }
                    ),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.render_bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        drop(render_pass);
    }
}
