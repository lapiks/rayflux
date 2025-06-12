use crate::{common::shapes::Plane, raytracer::cpu::{shapes::Hittable, Ray}};

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) {
        
    }
}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }
}
