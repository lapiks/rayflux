use crate::{common::{shapes::Sphere, Object}, raytracer::cpu::{intersections::{Intersection, Intersections}, shapes::Hittable, Ray}};


impl Hittable for Sphere {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let sphere_to_ray = ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        // -1.0 for radius*radius with radius = 1
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b*b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new()
        } 

        let sqrt_disc = discriminant.sqrt();
        let inv_denom = 1.0 / (2.0 * a);

        Intersections::new()
        .with_intersections(
            vec![
                Intersection::new(
                    (-b - sqrt_disc) * inv_denom, 
                    object
                ),
                Intersection::new(
                    (-b + sqrt_disc) * inv_denom, 
                    object
                )
            ]
        )
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }
}


#[cfg(test)]
mod tests {
    use glam::DVec3;
    use crate::raytracer::cpu::intersections::intersect;
    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            DVec3::new(0.0, 0.0, -5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            DVec3::new(0.0, 1.0, -5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t(), 5.0);
        assert_eq!(xs[1].t(), 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(
            DVec3::new(0.0, 2.0, -5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.count(), 0);
    }
    
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            DVec3::new(0.0, 0.0, 0.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            DVec3::new(0.0, 0.0, 5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    // #[test]
    // fn intersecting_a_scaled_sphere_with_a_ray() {
    //     let r = Ray::new(
    //         DVec3::new(0.0, 0.0, -5.0), 
    //         DVec3::new(0.0, 0.0, 1.0)
    //     );
    //     let s = Sphere::default();
    //     let o = Object::new(Shape::Sphere(s))
    //         .with_scale(2.0, 2.0, 2.0)
    //         .transform();
    //     let xs = o.intersect(&r);
    //     assert_eq!(xs.count(), 2);
    //     assert_eq!(xs.get(0).unwrap().t(), 3.0);
    //     assert_eq!(xs.get(1).unwrap().t(), 7.0);
    // } 

    // #[test]
    // fn intersecting_a_translated_sphere_with_a_ray() {
    //     let r = Ray::new(
    //         DVec3::new(0.0, 0.0, -5.0), 
    //         DVec3::new(0.0, 0.0, 1.0)
    //     );
    //     let s = Sphere::default();
    //     let o = Object::new(Shape::Sphere(s))
    //         .with_translation(5.0, 0.0, 0.0)
    //         .transform();
    //     let xs = o.intersect(&r);
    //     assert_eq!(xs.count(), 0);
    // } 

    // #[test]
    // fn normal_on_x_axis() {
    //     let s = Sphere::default();
    //     let n = s.normal_at(DVec3::new(1.0, 0.0, 0.0), 0.0, 0.0);
    //     assert_eq!(n, DVec3::new(1.0, 0.0, 0.0));
    // } 

    // #[test]
    // fn normal_on_y_axis() {
    //     let s = Sphere::default();
    //     let n = s.normal_at(DVec3::new(0.0, 1.0, 0.0), 0.0, 0.0);
    //     assert_eq!(n, DVec3::new(0.0, 1.0, 0.0));
    // } 

    // #[test]
    // fn normal_on_z_axis() {
    //     let s = Sphere::default();
    //     let n = s.normal_at(DVec3::new(0.0, 0.0, 1.0), 0.0, 0.0);
    //     assert_eq!(n, DVec3::new(0.0, 0.0, 1.0));
    // } 

    // #[test]
    // fn normal_at_nonaxial_point() {
    //     let s = Sphere::default();
    //     let n = s.normal_at(DVec3::new(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt()/3.0), 0.0, 0.0);
    //     assert!(n.abs_diff_eq(DVec3::new(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0), EPSILON));
    // } 

    // #[test]
    // fn the_normal_is_normalized() {
    //     let s = Sphere::default();
    //     let n = s.normal_at(DVec3::new(3.3_f64.sqrt() / 3.0, 3.3_f64.sqrt()/3.0, 3.3_f64.sqrt()/3.0), 0.0, 0.0);
    //     assert!(n.abs_diff_eq(n.normalize(), EPSILON));
    // } 

    // #[test]
    // fn computing_normal_on_a_translated_sphere() {
    //     let o = Object::new(Shape::Sphere(Sphere::default()))
    //         .with_translation(0.0, 1.0, 0.0)
    //         .transform();
    //     let n = o.normal_at(DVec3::new(0.0, 1.70711, -0.70711), 0.0, 0.0);
    //     assert!(n.abs_diff_eq(DVec3::new(0.0, 0.70711, -0.70711), EPSILON));
    // } 

    // #[test]
    // fn computing_normal_on_a_transformed_sphere() {
    //     let o = Object::new(Shape::Sphere(Sphere::default()))
    //         .with_rotation_z(PI / 5.0)
    //         .with_scale(1.0, 0.5, 1.0)
    //         .transform();
            
    //     let n = o.normal_at(DVec3::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0), 0.0, 0.0);
    //     assert!(n.abs_diff_eq(DVec3::new(0.0, 0.97014, -0.24254), EPSILON));
    // } 

    // #[test]
    // fn a_sphere_has_a_bounding_box() {
    //     let s = Sphere::default();
    //     let bb = s.bounds();
    //     assert_eq!(bb.min(), dvec3(-1.0, -1.0, -1.0));
    //     assert_eq!(bb.max(), dvec3(1.0, 1.0, 1.0));
    // }
}