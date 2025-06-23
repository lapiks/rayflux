use std::f64::EPSILON;

use glam::DVec3;

use crate::{common::{shapes::Cylinder, Object}, raytracer::cpu::{intersections::{HittableShape, Intersection, Intersections}, Ray}};

impl HittableShape for Cylinder {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        fn check_cap(ray: &Ray, t: f64) -> bool {
            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            (x * x + z * z) <= 1.0
        }

        fn intersect_caps<'a>(cylinder: &Cylinder, ray: &Ray, object: &'a Object) -> Intersections<'a> {
            if !cylinder.closed || f64::abs(ray.direction.y) < f64::EPSILON {
                return Intersections::new();
            }

            let mut xs = Vec::default();

            let t = (cylinder.min - ray.origin.y) / ray.direction.y;
            if check_cap(ray, t) {
                xs.push(
                    Intersection::new(t, object)
                );
            }

            let t = (cylinder.max - ray.origin.y) / ray.direction.y;
            if check_cap(ray, t) {
                xs.push(
                    Intersection::new(t, object)
                );
            }

            Intersections::new()
            .with_intersections(xs)
        }

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        if f64::abs(a) < f64::EPSILON {
            return intersect_caps(self, ray, object);
        }

        let b = 2.0 * ray.origin.x * ray.direction.x +
            2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.0;

        let disc = b*b - 4.0*a*c;
        if disc < 0.0 {
            return Intersections::new();
        }

        let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
        let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        let y0 = ray.origin.y + t0 * ray.direction.y;
        let mut xs = Vec::default();
        if self.min < y0 && y0 < self.max {
            xs.push(Intersection::new(t0, object));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.min < y1 && y1 <self.max {
            xs.push(Intersection::new(t1, object));
        }

        let mut intersections = Intersections::new()
        .with_intersections(xs);

        intersections.append(intersect_caps(self, ray, object));
        intersections
    }
    
    fn normal_at<'a>(&self, point: DVec3) -> DVec3 {
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.max - EPSILON {
            DVec3::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.min + EPSILON {
            DVec3::new(0.0, -1.0, 0.0)
        } else {
            DVec3::new(point.x, 0.0, point.z)
        }
    }
}