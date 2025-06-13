use crate::common::{Material, Transform};

#[derive(Default, PartialEq)]
pub struct Object {
    transform: Transform,
    material: Material,
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}