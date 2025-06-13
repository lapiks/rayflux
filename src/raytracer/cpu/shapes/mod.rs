use crate::{common::{shapes::Shape, Object}, raytracer::cpu::{intersections::Intersections, Ray}};

pub mod sphere;
pub mod plane;
pub mod cube;
pub mod cylinder;

pub use sphere::*;
pub use plane::*;
pub use cube::*;
pub use cylinder::*;

pub trait Hittable {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a>; 
}

impl Hittable for Shape {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray, object),
            Shape::Plane(plane) => plane.intersect(ray, object),
            Shape::Cube(cube) => cube.intersect(ray, object),
            Shape::Cylinder(cylinder) => cylinder.intersect(ray, object),
        }
    }
}