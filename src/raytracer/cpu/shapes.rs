use crate::common::shapes::{Cube, Cylinder, Plane, Shape, Sphere};

pub trait Hittable {
    fn intersect(&self); 
}

impl Hittable for Shape {
    fn intersect(&self) {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(),
            Shape::Plane(plane) => plane.intersect(),
            Shape::Cube(cube) => cube.intersect(),
            Shape::Cylinder(cylinder) => cylinder.intersect(),
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self) {
        todo!()
    }
}

impl Hittable for Plane {
    fn intersect(&self) {
        todo!()
    }
}

impl Hittable for Cube {
    fn intersect(&self) {
        todo!()
    }
}

impl Hittable for Cylinder {
    fn intersect(&self) {
        todo!()
    }
}