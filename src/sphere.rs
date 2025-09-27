use crate::{
    hittable::{HitRecord, Hittable, calculate_face_normal},
    math::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        let root = if r <= ray_tmin || r >= ray_tmax {
            let r2 = (h + sqrtd) / a;
            if r2 <= ray_tmin || r2 >= ray_tmax {
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
