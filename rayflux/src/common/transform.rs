use glam::{DMat4, DQuat, DVec3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transform {
    translation: DVec3,
    rotation: DQuat,
    scale: DVec3,
    matrix: DMat4,
    inverse_matrix: DMat4,
    inverse_transpose_matrix: DMat4,
    dirty: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Transform {
        translation: DVec3::ZERO,
        rotation: DQuat::IDENTITY,
        scale: DVec3::ONE,
        matrix: DMat4::IDENTITY,
        inverse_matrix: DMat4::IDENTITY,
        inverse_transpose_matrix: DMat4::IDENTITY,
        dirty: false,
    };

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_translation(translation: DVec3) -> Self {
        Self {
            translation,
            dirty: true,
            ..Default::default()
        }
    }

    pub fn from_rotation(rotation: DQuat) -> Self {
        Self {
            rotation,
            dirty: true,
            ..Default::default()
        }
    }

    pub fn from_scale(scale: DVec3) -> Self {
        Self {
            scale,
            dirty: true,
            ..Default::default()
        }
    }

    pub fn with_translation(mut self, translation: DVec3) -> Self {
        self.translation = translation;
        self.dirty = true;
        self
    }

    pub fn with_rotation(mut self, rotation: DQuat) -> Self {
        self.rotation = rotation;
        self.dirty = true;
        self
    }

    pub fn with_rotation_x(mut self, angle: f64) -> Self {
        self.rotation = DQuat::from_axis_angle(DVec3::X, angle);
        self.dirty = true;
        self
    }

    pub fn with_rotation_y(mut self, angle: f64) -> Self {
        self.rotation = DQuat::from_axis_angle(DVec3::Y, angle);
        self.dirty = true;
        self
    }

    pub fn with_rotation_z(mut self, angle: f64) -> Self {
        self.rotation = DQuat::from_axis_angle(DVec3::Z, angle);
        self.dirty = true;
        self
    }

    pub fn with_scale(mut self, scale: DVec3) -> Self {
        self.scale = scale;
        self.dirty = true;
        self
    }

    pub fn translate(&mut self, translation: DVec3) {
        self.translation += translation;
        self.dirty = true;
    }

    pub fn rotate(&mut self, rotation: DQuat) {
        self.rotation = rotation * self.rotation;
        self.dirty = true;
    }

    pub fn rotate_x(&mut self, angle: f64) {
        self.rotate(DQuat::from_rotation_x(angle));
    }

    pub fn rotate_y(&mut self, angle: f64) {
        self.rotate(DQuat::from_rotation_y(angle));
    }

    pub fn rotate_z(&mut self, angle: f64) {
        self.rotate(DQuat::from_rotation_z(angle));
    }

    pub fn left(&self) -> DVec3 {
        self.rotation * DVec3::X
    }

    pub fn right(&self) -> DVec3 {
        self.rotation * DVec3::NEG_X
    }

    pub fn up(&self) -> DVec3 {
        self.rotation * DVec3::Y
    }

    pub fn down(&self) -> DVec3 {
        self.rotation * DVec3::NEG_Y
    }

    pub fn forward(&self) -> DVec3 {
        self.rotation * DVec3::Z
    }

    pub fn backward(&self) -> DVec3 {
        self.rotation * DVec3::NEG_Z
    }

    pub fn matrix(&self) -> DMat4 {
        self.matrix
    }

    pub fn inverse_matrix(&self) -> DMat4 {
        self.inverse_matrix
    }

    pub fn inverse_transpose_matrix(&self) -> DMat4 {
        self.inverse_transpose_matrix
    }

    pub fn update_matrix(&mut self) {
        let t = DMat4::from_translation(self.translation);
        let r = DMat4::from_quat(self.rotation);
        let s = DMat4::from_scale(self.scale);
        self.matrix = t * r * s;
        self.inverse_matrix = self.matrix.inverse();
        self.inverse_transpose_matrix = self.inverse_matrix.transpose();
        self.dirty = false;
    }
}