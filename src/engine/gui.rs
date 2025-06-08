use egui_wgpu::ScreenDescriptor;
use winit::{event::WindowEvent, window::Window};

use super::{Frame, Renderer};

pub struct GuiRenderer {
    egui_renderer: egui_wgpu::Renderer,
    egui_context: egui::Context,
    egui_state: egui_winit::State,
}

impl GuiRenderer {
    pub fn new(renderer: &Renderer) -> Self {
        let egui_renderer = egui_wgpu::Renderer::new(
            renderer.device(), 
            renderer.surface_format(), 
            None, 
            1, 
            false
        );

        let egui_context = egui::Context::default();
        let viewport_id = egui_context.viewport_id();

        let egui_state = egui_winit::State::new(
            egui_context.clone(), 
            viewport_id, 
            renderer.window(), 
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
        frame: &mut Frame, 
        renderer: &Renderer,
        ui_fn: F,
    ) 
    where F: FnMut(&egui::Context) {
        let window = renderer.window();

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
                    renderer.device(), 
                    renderer.queue(), 
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
            renderer.device(), 
            renderer.queue(), 
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