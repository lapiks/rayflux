use crate::raytracer::{RaytracerImpl, RaytracerOutput};

pub mod canvas;
pub mod ray;
pub mod shapes;

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