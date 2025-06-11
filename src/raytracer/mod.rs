use clap::ValueEnum;
use glam::UVec2;

use crate::{output::RaytracerOutput, raytracer::{cpu::CpuRaytracer, gpu::GpuRaytracer}};

pub mod cpu;
pub mod gpu;

pub use cpu::*;
pub use gpu::*;

#[derive(ValueEnum, Clone, Debug)]
pub enum RaytracerType {
    Cpu,
    Gpu,
}

impl Default for RaytracerType {
    fn default() -> Self {
        Self::Gpu
    }
}

pub enum Raytracer {
    Cpu(CpuRaytracer),
    Gpu(GpuRaytracer),
}

pub trait RaytracerImpl {
    fn render(&self);
    fn output(&self) -> &RaytracerOutput;
}

impl RaytracerImpl for Raytracer {
    fn render(&self) {
        match self {
            Raytracer::Cpu(cpu) => cpu.render(),
            Raytracer::Gpu(gpu) => gpu.render(),
        }
    }
    
    fn output(&self) -> &RaytracerOutput {
        match self {
            Raytracer::Cpu(cpu) => cpu.output(),
            Raytracer::Gpu(gpu) => gpu.output(),
        }
    }
}

pub struct RenderParams {
    /// Rendering size
    size: UVec2,
}