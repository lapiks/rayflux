use crate::{common::shapes::Cylinder, raytracer::cpu::{shapes::Hittable, Ray}};

impl Hittable for Cylinder {
    fn intersect(&self, ray: &Ray) {
        
    }
}

impl Cylinder {
    pub fn new() -> Self {
        Self::default()
    }
}
