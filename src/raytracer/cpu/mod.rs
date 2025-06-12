use crate::raytracer::{RaytracerImpl, RaytracerOutput};

pub mod canvas;

#[derive(Default)]
pub struct CpuRaytracer {
    
}

impl RaytracerImpl for CpuRaytracer {
    fn output(&self) -> RaytracerOutput {
        RaytracerOutput::Image
    }
}

impl CpuRaytracer {
    pub fn render(&self) {
        todo!()
    }
}