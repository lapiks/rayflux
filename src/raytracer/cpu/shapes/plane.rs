use std::f64::EPSILON;

use crate::{common::{shapes::Plane, Object}, raytracer::cpu::{intersections::{Intersection, Intersections}, shapes::Hittable, Ray}};

impl Hittable for Plane {
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
}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }
}
