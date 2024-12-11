pub mod sphere;
use std::path::Iter;

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hits: &mut HitRecord) -> bool;
}
pub struct HitRecord {
    point: Point,
    normal: Vec3,
    hit_t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

struct Hittable_List {
    list: Vec<Box<dyn Hittable>>,
}

impl Hittable_List {
    fn new() -> Self {
        Hittable_List { list: Vec::new() }
    }

    fn get(&self, index: usize) -> Option<&Box<dyn Hittable>> {
        self.list.get(index)
    }

    fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }

    fn clear(&mut self) {
        self.list.clear();
    }
}

impl Iterator for Hittable_List {
    type Item = Box<dyn Hittable>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hittable) = self.list.pop() {
            return Some(hittable);
        } else {
            None
        }
    }
}

impl Hittable for Hittable_List {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hits: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        for hittable in self.list.iter() {
            if hittable.hit(ray, t_min, t_max, hits) {
                hit_anything = true;
            }
        }
        hit_anything
    }
}
