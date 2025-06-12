use crate::{common::shapes::Cube, raytracer::cpu::{shapes::Hittable, Ray}};

impl Hittable for Cube {
    fn intersect(&self, ray: &Ray) {
        
    }
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}
