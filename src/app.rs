use std::sync::Arc;

use glam::{UVec2, Vec2};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::{ElementState, MouseScrollDelta, WindowEvent}, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::{engine::{GpuContext, GuiRenderer, Inputs, Renderer, Time, World}, features::UserInterface};

pub struct AppContext<'a> {
    pub time: &'a Time,
}

#[derive(Default)]
pub struct App {
    world: World,
    context: Option<GpuContext>,
    renderer: Option<Renderer>,
    gui_renderer: Option<GuiRenderer>,
    gui: UserInterface,
    inputs: Inputs,
    time: Time,
}

impl App {
    /// Begin of frame phase
    fn begin_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        self.world.init(context);
    }

    /// Game logic update phase
    fn update_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        self.time.tick();
        self.world.update(context);
    }

    /// Rendering phase
    fn render_phase(&mut self) {
        let context = self.context.as_mut().unwrap();
        let renderer = self.renderer.as_mut().unwrap();
        let gui_renderer = self.gui_renderer.as_mut().unwrap();

        match context.begin_frame() {
            Ok(mut frame) => {
                renderer.pre_render(context, &mut self.world);
                renderer.render(&mut frame);

                let app_ctx = AppContext {
                    time: &self.time,
                };

                // Render UI
                gui_renderer.render(
                    &mut frame,
                    context, 
                    |ctx| self.gui.run_ui(ctx, &app_ctx)
                );

                context.end_frame(frame);
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
        let context = self.context.as_mut().unwrap();
        context.window().request_redraw();
    }

    /// Resize callback
    fn on_resize(&mut self, size: UVec2) {
        let camera = self.world.camera_mut();
        camera.update_aspect_ratio(size);

        let context = self.context.as_mut().unwrap();
        context.resize(size);

        let renderer = self.renderer.as_mut().unwrap();
        renderer.resize(context.device(), size);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        env_logger::init();

        // Create window
        let attributes = Window::default_attributes()
            .with_title("Rayflux")
            .with_inner_size(LogicalSize::new(1600.0, 900.0))
            .with_min_inner_size(LogicalSize::new(100.0, 100.0));

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .unwrap()
        );

        // create gpu context
        let context = pollster::block_on(GpuContext::new(window.clone()));
        
        self.renderer = Some(Renderer::new(&context, &self.world));
        self.gui_renderer = Some(GuiRenderer::new(&context));
        self.context = Some(context);

        let mut user_interface = UserInterface::default();
        user_interface.init();
        self.gui = user_interface;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let context = self.context.as_mut().unwrap();
        let gui_renderer = self.gui_renderer.as_mut().unwrap();
        gui_renderer.handle_event(&event, context.window());

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