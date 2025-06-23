use std::cmp::Ordering;

use glam::DVec3;

use crate::{common::{Object, Scene}, raytracer::cpu::Ray};

pub trait Hittable {
    /// Intersect an object with a ray and return the resulting intersections
    fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a>; 
    fn normal_at<'a>(&self, point: DVec3) -> DVec3; 
}

pub trait HittableShape {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a>; 
    fn normal_at<'a>(&self, point: DVec3) -> DVec3; 
}

impl Hittable for Object {
    fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        let local_ray = ray.transform(&self.transform().inverse_matrix());
        self.shape().intersect(&local_ray, self)
    }

    fn normal_at<'a>(&self, point: glam::DVec3) -> DVec3 {
        let transform = self.transform();
        let local_point = transform.inverse_matrix().transform_point3(point);
        let local_normal = self.shape().normal_at(local_point);
        transform.inverse_transpose_matrix().transform_vector3(local_normal).normalize()
    }
}

pub fn intersect_scene<'a>(ray: &Ray, scene: &'a Scene) -> Intersections<'a> {
    let mut intersections = Intersections::new();
    for object in scene.objects().iter() {
       intersections.append(object.intersect(ray));
    }
    intersections.sort()
}

pub trait HitPredicate {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool>;
    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool>;
}

pub struct StandardHit {}

impl HitPredicate for StandardHit {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool> {
       Box::new(|i| i.t >= 0.0)
    }

    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool> {
        Box::new(|i| i.t >= 0.0)
    }
}

pub struct ShadowHit {}

impl HitPredicate for ShadowHit {
    fn hit_predicate(&self) -> Box<dyn FnMut(&&Intersection<'_>) -> bool> {
        //Box::new(|i| i.object.shadow() && i.t >= 0.0)
        Box::new(|i| i.t >= 0.0)
    }

    fn hit_index_predicate(&self) -> Box<dyn FnMut(&Intersection<'_>) -> bool> {
        Box::new(|i| i.t >= 0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Self {
            t,
            object,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Object {
        &self.object
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t < other.t {
            return Ordering::Less;
        }
        else if self.t > other.t {
            return Ordering::Greater;
        }
        else {
            return Ordering::Equal;
        }
    }
}

impl<'a> Eq for Intersection<'a> {}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {  
            intersections: Vec::default()
        }
    }

    pub fn from_capacity(size: usize) -> Self {
        Self {
            intersections: Vec::with_capacity(size)
        }
    }

    pub fn with_intersections(mut self, intersections: Vec<Intersection<'a>>) -> Self {
        self.intersections = intersections;
        self
    }

    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.intersections.push(intersection);
    }

    pub fn append(&mut self, other: Intersections<'a>) {
        self.intersections.append(&mut other.move_all());
    }

    pub fn sort(mut self) -> Self {
        self.intersections.sort();
        self
    }

    pub fn hit(&self, predicate: impl HitPredicate) -> Option<&Intersection> {
        self.intersections.iter().find(predicate.hit_predicate())
    }

    pub fn hit_index(&self, predicate: impl HitPredicate) -> Option<usize> {
        self.intersections.iter().position(predicate.hit_index_predicate())
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.intersections.get(index)
    }

    pub fn move_all(self) -> Vec<Intersection<'a>> {
        self.intersections
    }

    pub fn get_all(&self) -> &Vec<Intersection<'a>> {
        &self.intersections
    }
}

impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    // row major
    fn index(&self, index: usize) -> &Intersection<'a> {
        &self.intersections[index]
    }
}

/// All the informations about an intersection
pub struct IntersectionInfos<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: DVec3,
    pub normal: DVec3,
}

impl<'a> IntersectionInfos<'a> {
    pub fn new(intersections: &'a Intersections<'a>, intersection_index: usize, ray: &Ray) -> Self {
        let intersection = intersections.get(intersection_index).unwrap();
        let t = intersection.t;
        let point = ray.at(t);
        let eyev = -ray.direction;
        let object = intersection.object;
        let mut normal = object.normal_at(point);
        let mut inside = false;
        if normal.dot(eyev) < 0.0 {
            inside = true;
            normal = -normal;
        }

        Self {
            t,
            object,
            point,
            normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // An intersection encapsulate t and object
    #[test]
    fn new_intersection() {
        let o = Object::new_sphere();
        let i = Intersection::new(3.5, &o);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &o);
    }

    #[test]
    fn intersections_is_empty_by_default() {
        let xs = Intersections::new();
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn aggregating_intersections() {
        let o = Object::new_sphere();
        let i1 = Intersection::new(1.0, &o);
        let i2 = Intersection::new(2.0, &o);
        let xs = Intersections::new().with_intersections(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0), Some(&i1));
        assert_eq!(xs.get(1), Some(&i2));
    }
    
    #[test]
    fn aggregating_intersections_with_pushes() {
        let o = Object::new_sphere();
        let i1 = Intersection::new(1.0, &o);
        let i2 = Intersection::new(2.0, &o);
        let mut xs = Intersections::new();
        xs.push(i1.clone());
        xs.push(i2.clone());
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0), Some(&i1));
        assert_eq!(xs.get(1), Some(&i2));
    }
}