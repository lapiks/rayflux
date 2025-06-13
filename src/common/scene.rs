use crate::common::{Camera, GpuContext, Object};

#[derive(Default)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Object>,
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

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }
}