#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

#[derive(Debug, Default, PartialEq)]
pub struct Sphere {}

#[derive(Debug, Default, PartialEq)]
pub struct Plane {}

#[derive(Debug, Default, PartialEq)]
pub struct Cube {}

#[derive(Debug, Default, PartialEq)]
pub struct Cylinder {
    pub min: f64,
    pub max: f64,
    pub closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self::default()
    }
}