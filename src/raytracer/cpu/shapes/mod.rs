use crate::{common::shapes::Shape, raytracer::cpu::Ray};

pub mod sphere;
pub mod plane;
pub mod cube;
pub mod cylinder;

pub use sphere::*;
pub use plane::*;
pub use cube::*;
pub use cylinder::*;

pub trait Hittable {
    fn intersect(&self, ray: &Ray); 
}

impl Hittable for Shape {
    fn intersect(&self, ray: &Ray) {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),
            Shape::Cube(cube) => cube.intersect(ray),
            Shape::Cylinder(cylinder) => cylinder.intersect(ray),
        }
    }
}