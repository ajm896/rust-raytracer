pub mod sphere;

use interval::Interval;

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &interval::Interval, hits: &mut HitRecord) -> bool;
}
#[derive(Debug, Clone, Copy)]
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
            -(outward_normal.clone())
        }
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal.clone()
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Point::zero(),
            normal: Vec3::zero(),
            hit_t: 0.,
            front_face: false,
        }
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Iterator for HittableList {
    type Item = Box<dyn Hittable>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hittable) = self.list.pop() {
            return Some(hittable);
        } else {
            None
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_range.range.end;
        let mut tmp_hit_rec: HitRecord = HitRecord::default();
        for hittable in self.list.iter() {
            if hittable.hit(
                ray,
                &Interval {
                    range: t_range.range.start..closest_so_far,
                },
                &mut tmp_hit_rec,
            ) {
                hit_anything = true;
                closest_so_far = tmp_hit_rec.hit_t;
                *hit_record = tmp_hit_rec.clone();
            }
        }
        hit_anything
    }
}

pub mod interval {
    use core::f64;
    use std::ops::Range;

    pub struct Interval {
        pub range: Range<f64>,
    }

    impl Interval {
        pub fn new(range: &Range<f64>) -> Interval {
            Interval {
                range: range.clone(),
            }
        }
        pub fn surrounds(&self, value: &f64) -> bool {
            (self.range.start - 1. ..self.range.end + 1.).contains(value)
        }
    }
}
