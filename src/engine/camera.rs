use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::engine::Renderer;

const DEFAULT_POSITION: Vec3 = Vec3::new(2.5, 2.5, 2.5);

pub struct Camera {
    position: Vec3,
    target: Vec3,
    field_of_view: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    uniform: CameraUniform,
    uniform_buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
    dirty: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            position: DEFAULT_POSITION, 
            target: Vec3::ZERO, 
            field_of_view: std::f32::consts::FRAC_PI_4,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
            uniform: CameraUniform::default(),
            uniform_buffer: None,
            bind_group: None,
            dirty: false,
        }
    }
}

impl Camera {
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.dirty = true;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec3 {
        self.dirty = true;
        &mut self.position
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.dirty = true;
    }    
    
    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }

    pub fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        self.bind_group.as_ref()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_clean(&mut self) {
        self.dirty = false;
    }

    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.dirty = true;
    }

    pub fn update_matrix(&mut self) {
        let view = Mat4::look_at_rh(self.position, self.target, Vec3::Y);
        let proj = Mat4::perspective_rh_gl(self.field_of_view, self.aspect_ratio, self.near, self.far);
        let view_proj = proj * view;
        self.uniform.view_proj = view_proj.to_cols_array_2d();
    }

    pub fn create_uniform_buffer(&mut self, renderer: &Renderer) {
        self.uniform_buffer = Some(
            renderer.device().create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("camera uniform buffer"),
                    contents: bytemuck::bytes_of(&self.uniform),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }
            )
        );
    }

    pub fn update_uniform_buffer(&self, renderer: &Renderer) {
        let buffer = self.uniform_buffer
            .as_ref()
            .expect("uniform buffer missing");

        renderer.queue().write_buffer(
            &buffer, 
            0, 
            bytemuck::bytes_of(&self.uniform)
        );
    }

    pub fn create_bind_group(&mut self, renderer: &Renderer) {
        let buffer = self.uniform_buffer
            .as_ref()
            .expect("uniform buffer missing");

        self.bind_group = Some(
            renderer.device().create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &renderer.camera_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("Camera::bind_group"),
            })
        ) 
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self { 
            view_proj: Mat4::IDENTITY.to_cols_array_2d(), 
        }
    }
}