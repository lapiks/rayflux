use glam::DVec3;

use crate::{common::{shapes::Cube, Object}, raytracer::cpu::{intersections::{Intersection, Intersections}, shapes::Hittable, Ray}};

impl Hittable for Cube {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let tmin_numerator = -1.0 - origin;
            let tmax_numerator = 1.0 - origin;

            let mut tmin;
            let mut tmax;
            if direction.abs() >= f64::EPSILON {
                tmin = tmin_numerator / direction;
                tmax = tmax_numerator / direction;
            } else {
                tmin = tmin_numerator * f64::INFINITY;
                tmax = tmax_numerator * f64::INFINITY;
            }

            if tmin > tmax {
                std::mem::swap(&mut tmin, &mut tmax);
            }

            (tmin, tmax)
        }

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = f64::max(xtmin, f64::max(ytmin, ztmin));
        let tmax = f64::min(xtmax, f64::min(ytmax, ztmax));

        if tmin > tmax {
            Intersections::new()
        } else {
            Intersections::new()
            .with_intersections(
                vec![
                    Intersection::new(tmin, object),
                    Intersection::new(tmax, object)
                ]
            )
        }
    }
    
    fn normal_at<'a>(&self, point: DVec3) -> DVec3 {
        todo!()
    }
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}
