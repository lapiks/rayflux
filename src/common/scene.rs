use crate::common::{Camera, GpuContext};

#[derive(Default)]
pub struct Scene {
    camera: Camera,
}

impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn init(&mut self, context: &mut GpuContext) {

    }

    pub fn update(&mut self, context: &GpuContext) {

    }
}