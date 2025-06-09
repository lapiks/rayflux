use std::sync::Arc;

use glam::UVec2;
use winit::window::Window;

pub struct Frame {
    pub surface_texture: wgpu::SurfaceTexture,
    pub surface_view: wgpu::TextureView,
    pub size: UVec2,
    pub command_encoder: wgpu::CommandEncoder,
}

/// wgpu context
pub struct GpuContext {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    size: UVec2,
}

impl GpuContext {
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

        let renderer = GpuContext {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
        };

        // Configure surface for the first time
        renderer.configure_surface();

        renderer
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

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn resize(&mut self, new_size: UVec2) {
        self.size = new_size;

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
            size: self.size,
            command_encoder,
        })
    }

    pub fn end_frame(&self, frame: Frame) {
        // Submit the command in the queue to execute
        self.queue.submit([frame.command_encoder.finish()]);
        self.window.pre_present_notify();
        frame.surface_texture.present();
    }
}
