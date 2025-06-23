use glam::{UVec2, Vec3};

use crate::common::color::Color;

const DEFAULT_POSITION: Vec3 = Vec3::new(0.0, 0.0, 5.0);

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    field_of_view: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    background: Color,
    dirty: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            position: DEFAULT_POSITION, 
            target: Vec3::ZERO, 
            up: Vec3::Y,
            field_of_view: std::f32::consts::FRAC_PI_4,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
            background: Color::BLACK,
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

    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
        self.dirty = true;
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn set_field_of_view(&mut self, fov: f32) {
        self.field_of_view = fov;
        self.dirty = true;
    }

    pub fn field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.dirty = true;
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.dirty = true;
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_clean(&mut self) {
        self.dirty = false;
    }

    pub fn update_aspect_ratio(&mut self, size: UVec2) {
        self.aspect_ratio = size.x as f32 / size.y as f32;
        self.dirty = true;
    }
}