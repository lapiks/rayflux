use crate::engine::{Camera, Renderer};

#[derive(Default)]
pub struct World {
    camera: Camera,
}

impl World {
    pub fn init(&mut self, renderer: &mut Renderer) {
        self.camera.create_uniform_buffer(renderer);
    }

    pub fn update(&mut self, renderer: &Renderer) {
        if self.camera.is_dirty() {
            self.camera.update_matrix();
            self.camera.update_uniform_buffer(renderer);
            self.camera.set_clean();
        }
    }
}