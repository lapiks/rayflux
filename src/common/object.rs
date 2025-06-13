use crate::common::{shapes::{Cube, Cylinder, Plane, Shape, Sphere}, Material, Transform};

#[derive(Debug, PartialEq)]
pub struct Object {
    shape: Shape,
    transform: Transform,
    material: Material,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn new_sphere() -> Self {
        Self {
            shape: Shape::Sphere(Sphere::default()),
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn new_plane() -> Self {
        Self {
            shape: Shape::Plane(Plane::default()),
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn new_cube() -> Self {
        Self {
            shape: Shape::Cube(Cube::default()),
            transform: Transform::default(),
            material: Material::default(),
        }
    }

    pub fn new_cylinder() -> Self {
        Self {
            shape: Shape::Cylinder(Cylinder::default()),
            transform: Transform::default(),
            material: Material::default(),
        }
    }
}