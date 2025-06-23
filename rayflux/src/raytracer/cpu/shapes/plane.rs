use std::f64::EPSILON;

use glam::DVec3;

use crate::{common::{shapes::Plane, Object}, raytracer::cpu::{intersections::{HittableShape, Intersection, Intersections}, Ray}};

impl HittableShape for Plane {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if ray.direction.y.abs() > EPSILON {
            xs.push(
                Intersection::new(
                    -ray.origin.y / ray.direction.y,
                    object
                )
            );
        }

        xs
    }
    
    fn normal_at<'a>(&self, point: DVec3) -> DVec3 {
        DVec3::new(0.0, 1.0, 0.0)
    }
}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }
}
