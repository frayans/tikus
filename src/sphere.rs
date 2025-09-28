use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable, calculate_face_normal},
    math::Point3,
    ray::Ray,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // make sure root is between an acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let (front_face, normal) = calculate_face_normal(ray, (p - self.center) / self.radius);
        let record = HitRecord {
            point: p,
            t: root,
            front_face,
            normal,
        };

        Some(record)
    }
}
