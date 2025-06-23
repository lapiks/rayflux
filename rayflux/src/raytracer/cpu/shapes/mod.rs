use crate::{common::{shapes::Shape, Object}, raytracer::cpu::{intersections::{HittableShape, Intersections}, Ray}};

pub mod sphere;
pub mod plane;
pub mod cube;
pub mod cylinder;

use glam::DVec3;
pub use sphere::*;
pub use plane::*;
pub use cube::*;
pub use cylinder::*;

impl HittableShape for Shape {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray, object),
            Shape::Plane(plane) => plane.intersect(ray, object),
            Shape::Cube(cube) => cube.intersect(ray, object),
            Shape::Cylinder(cylinder) => cylinder.intersect(ray, object),
        }
    }
    
    fn normal_at<'a>(&self, point: DVec3) -> DVec3 {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(point),
            Shape::Plane(plane) => plane.normal_at(point),
            Shape::Cube(cube) => cube.normal_at(point),
            Shape::Cylinder(cylinder) => cylinder.normal_at(point),
        }
    }
}