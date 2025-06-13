use std::cmp::Ordering;

use crate::common::Object;

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

#[derive(Clone, PartialEq)]
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
