use crate::raytracer::RaytracerImpl;

#[derive(Default)]
pub struct CpuRaytracer {
    
}

impl RaytracerImpl for CpuRaytracer {
    fn render(&self) {
        todo!()
    }
    
    fn output(&self) -> &crate::output::RaytracerOutput {
        todo!()
    }
}