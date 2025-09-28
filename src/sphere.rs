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
        let oc = self.center + ray.origin();
        let a = ray.direction().mag2();
        let h = ray.direction().dot(oc);
        let c = oc.mag2() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // make sure root is between an acceptable range
        let r = (h - sqrtd) / a;
        let root = if !ray_t.contains(&r) {
            let r2 = (h + sqrtd) / a;
            if !ray_t.contains(&r2) {
                return None;
            }
            r2
        } else {
            r
        };

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
