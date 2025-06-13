use core::fmt;

use clap::ValueEnum;
use glam::UVec2;

use crate::{common::Texture, raytracer::{cpu::CpuRaytracer, gpu::GpuRaytracer}};

pub mod cpu;
pub mod gpu;

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

impl fmt::Display for RaytracerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RaytracerType::Cpu => write!(f, "CPU"),
            RaytracerType::Gpu => write!(f, "GPU"),
        }
    }
}

pub enum Raytracer {
    Cpu(CpuRaytracer),
    Gpu(GpuRaytracer),
}

pub trait RaytracerImpl {
    fn output(&self) -> RaytracerOutput;
}

impl RaytracerImpl for Raytracer {    
    fn output(&self) -> RaytracerOutput {
        match self {
            Raytracer::Cpu(cpu) => cpu.output(),
            Raytracer::Gpu(gpu) => gpu.output(),
        }
    }
}

pub enum RaytracerOutput<'a> {
    WgpuTexture(&'a Texture),
    Image,
}