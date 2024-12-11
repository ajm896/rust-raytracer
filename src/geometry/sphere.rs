use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

use super::{HitRecord, Hittable};
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - ray.origin();
        let a = ray.direction().squared_length();
        let h: f64 = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root: f64 = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        hit_record.hit_t = root;
        hit_record.point = ray.at(hit_record.hit_t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
