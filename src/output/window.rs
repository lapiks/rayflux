use std::sync::Arc;

use egui_wgpu::ScreenDescriptor;
use glam::{UVec2, Vec2};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::{ElementState, MouseScrollDelta, WindowEvent}, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::{common::{Frame, GpuContext, Inputs, Scene, Texture, Time}, features::UserInterface, output::RaytracerOutput, raytracer::{cpu::CpuRaytracer, gpu::GpuRaytracer, Raytracer, RaytracerImpl, RaytracerType}};

pub struct AppContext<'a> {
    pub time: &'a Time,
}

#[derive(Default)]
pub struct WindowApp {
    raytracer_type: RaytracerType,
    raytracer: Option<Raytracer>,
    scene: Scene,
    context: Option<GpuContext>,
    renderer: Option<FullScreenRenderer>,
    gui_renderer: Option<GuiRenderer>,
    gui: UserInterface,
    inputs: Inputs,
    time: Time,
}

impl WindowApp {
    pub fn new(raytracer_type: RaytracerType) -> Self {
        Self {
            raytracer_type,
            ..Default::default()
        }
    }

    /// Begin of frame phase
    fn begin_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        self.scene.init(context);
    }

    /// Game logic update phase
    fn update_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        self.time.tick();
        self.scene.update(context);
    }

    /// Rendering phase
    fn render_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        let renderer = self.renderer.as_mut().unwrap();
        let gui_renderer = self.gui_renderer.as_mut().unwrap();

        match renderer.begin_frame(context) {
            Ok(mut frame) => {
                renderer.render(&mut frame);

                let app_ctx = AppContext {
                    time: &self.time,
                };

                // Render UI
                gui_renderer.render(
                    &renderer.window,
                    &mut frame,
                    context, 
                    |ctx| self.gui.run_ui(ctx, &app_ctx)
                );

                renderer.end_frame(context, frame);
            },
            Err(wgpu::SurfaceError::Timeout) => {
                // This happens when the a frame takes too long to present
                log::warn!("Surface timeout");
            },
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                // Reconfigure the surface if it's lost or outdated
                log::warn!("Lost or outdated surface");
            },
            Err(wgpu::SurfaceError::OutOfMemory) => {
                // The system is out of memory, we should probably quit
                log::error!("OutOfMemory");
            },
            Err(_) => {
                log::warn!("Generic error");
            },
        }
    }

    /// End of frame phase
    fn end_phase(&mut self) {
        self.inputs.reset();
        let renderer = self.renderer.as_mut().unwrap();
        renderer.window().request_redraw();
    }

    /// Resize callback
    fn on_resize(&mut self, size: UVec2) {
        let camera = self.scene.camera_mut();
        camera.update_aspect_ratio(size);

        let context = self.context.as_mut().unwrap();
        let renderer = self.renderer.as_mut().unwrap();
        renderer.resize(context.device(), size);
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        env_logger::init();

        let default_size = UVec2::new(1600, 800);

        // Create window
        let attributes = Window::default_attributes()
            .with_title("Rayflux")
            .with_inner_size(LogicalSize::new(default_size.x, default_size.y))
            .with_min_inner_size(LogicalSize::new(100.0, 100.0));

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .unwrap()
        );

        // Create gpu context
        let context = pollster::block_on(GpuContext::new());

        // Create Raytracer
        let raytracer = match self.raytracer_type {
            RaytracerType::Cpu => Raytracer::Cpu(CpuRaytracer::default()),
            RaytracerType::Gpu => Raytracer::Gpu(GpuRaytracer::new(&context, &self.scene, default_size)),
        };

        let output = raytracer.output();

        // Create full screen triangle renderer
        let renderer = FullScreenRenderer::new(window.clone(), &context, &output);

        // Create gui renderer
        let gui_renderer = GuiRenderer::new(&window, &context, renderer.surface_format);

        // Create graphical user interface
        let mut user_interface = UserInterface::default();
        user_interface.init();

        self.raytracer = Some(raytracer);
        self.renderer = Some(renderer);
        self.context = Some(context);
        self.gui = user_interface;
        self.gui_renderer = Some(gui_renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let renderer = self.renderer.as_mut().unwrap();
        let gui_renderer = self.gui_renderer.as_mut().unwrap();
        gui_renderer.handle_event(&event, &renderer.window);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.begin_phase();
                self.update_phase();
                self.render_phase();                
                self.end_phase();                
            },
            WindowEvent::Resized(size) => {
                self.on_resize(UVec2::new(size.width, size.height));
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.inputs.on_mouse_move(Vec2::new(position.x as f32, position.y as f32));
            },
            WindowEvent::MouseWheel { device_id, delta, phase } => {
                match delta {
                    MouseScrollDelta::LineDelta(delta, _) => self.inputs.on_mouse_wheel(delta),
                    MouseScrollDelta::PixelDelta(_) => todo!(),
                }
            },
            WindowEvent::MouseInput { device_id, state, button } => match state {
                ElementState::Pressed => self.inputs.on_mouse_button_down(button),
                ElementState::Released => self.inputs.on_mouse_button_up(button),
            },
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => match event.physical_key {
                winit::keyboard::PhysicalKey::Code(key) => {
                    match event.state {
                        ElementState::Pressed => self.inputs.on_key_down(key),
                        ElementState::Released => self.inputs.on_key_up(key),
                    }
                },
                winit::keyboard::PhysicalKey::Unidentified(_) => println!("An unidentified key has been pressed"),
            },
            WindowEvent::ModifiersChanged(mods) => {
                self.inputs.set_modifiers(mods.state());
            },
            _ => (),
        }
    }
}

struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub image_bind_group: wgpu::BindGroup,
}

/// Render a texture to a fullscreen triangle
pub struct FullScreenRenderer {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    size: UVec2,
    render_pipeline: RenderPipeline,
}

impl FullScreenRenderer {
    pub fn new(window: Arc<Window>, context: &GpuContext, raytracer_output: &RaytracerOutput) -> Self {
        let size = window.inner_size();

        let surface = context.instance().create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&context.adapter());
        let surface_format = cap.formats[0];

        let size = UVec2 { x: size.width, y: size.height };     

        let device = context.device();

        let render_target = match raytracer_output {
            RaytracerOutput::WgpuTexture(texture) => texture,
            RaytracerOutput::Image => todo!(),
        };
        
        let render_pipeline = Self::create_render_pipeline(&device, surface_format, render_target);

        let renderer = FullScreenRenderer { 
            window,
            surface,
            surface_format,
            size,
            render_pipeline 
        };

        // Configure surface for the first time
        renderer.configure_surface(device);

        renderer
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_format
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: UVec2) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface(device);
    }

    fn configure_surface(&self, device: &wgpu::Device) {
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
        self.surface.configure(device, &surface_config);
    }

    fn create_render_pipeline(device: &wgpu::Device, surface_format: wgpu::TextureFormat, render_target: &Texture) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("render.wgsl").into()),
        });

        let image_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image bind group layout"),
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
                bind_group_layouts: &[&image_bind_group_layout],
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

        let image_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image bind group"),
            layout: &image_bind_group_layout,
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
        });

        RenderPipeline {
            pipeline,
            image_bind_group,
        }
    }

    pub fn begin_frame(&self, context: &GpuContext) -> Result<Frame, wgpu::SurfaceError>{
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
        let command_encoder = context.device().create_command_encoder(&Default::default());

        Ok(Frame {
            surface_texture,
            surface_view,
            size: self.size,
            command_encoder,
        })
    }

    pub fn end_frame(&self, context: &GpuContext, frame: Frame) {
        // Submit the command in the queue to execute
        context.queue().submit([frame.command_encoder.finish()]);
        self.window.pre_present_notify();
        frame.surface_texture.present();
    }

    pub fn render(&mut self, frame: &mut Frame) {
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
        render_pass.set_bind_group(0, &self.render_pipeline.image_bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        drop(render_pass);
    }
}

pub struct GuiRenderer {
    egui_renderer: egui_wgpu::Renderer,
    egui_context: egui::Context,
    egui_state: egui_winit::State,
}

impl GuiRenderer {
    pub fn new(window: &Window, context: &GpuContext, surface_format: wgpu::TextureFormat) -> Self {
        let egui_renderer = egui_wgpu::Renderer::new(
            context.device(), 
            surface_format, 
            None, 
            1, 
            false
        );

        let egui_context = egui::Context::default();
        let viewport_id = egui_context.viewport_id();

        let egui_state = egui_winit::State::new(
            egui_context.clone(), 
            viewport_id, 
            window, 
            None, 
            None,
            None
        );

        Self {
            egui_renderer,
            egui_context,
            egui_state,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window) {
        let _ = self.egui_state.on_window_event(window, event);
    }

    /// Render gui by executing a given ui function
    pub fn render<F>(
        &mut self, 
        window: &Window,
        frame: &mut Frame, 
        context: &GpuContext,
        ui_fn: F,
    ) 
    where F: FnMut(&egui::Context) {
        let raw_input = self.egui_state.take_egui_input(window);
        let full_output = self.egui_context.run(raw_input, ui_fn);

        let clipped_primitives = self.egui_context
            .tessellate(
                full_output.shapes, 
                full_output.pixels_per_point
            );

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer
                .update_texture(
                    context.device(), 
                    context.queue(), 
                    *id, 
                    &image_delta
                );
        }

        let size = window.inner_size();
        let screen_desc = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        self.egui_renderer.update_buffers(
            context.device(), 
            context.queue(), 
            &mut frame.command_encoder,
            &clipped_primitives,
            &screen_desc,
        );

        {
            let render_pass = frame.command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Gui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
    
            self.egui_renderer.render(
                &mut render_pass.forget_lifetime(),
                &clipped_primitives[..],
                &screen_desc,
            );
        }
        
        for x in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(x);
        }
    }
}