pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

#[derive(Default)]
pub struct Sphere {
    
}

#[derive(Default)]
pub struct Plane {

}

#[derive(Default)]
pub struct Cube {
    
}

#[derive(Default)]
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