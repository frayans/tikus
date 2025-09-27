use crate::{
    math::{Interval, Point3, Vec3},
    ray::Ray,
};

pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::zero(),
            t: 0.0,
            normal: Vec3::zero(),
            front_face: false,
        }
    }
}

/// Calculates the `front_face: bool` and `normal: Vec3` of a surface
pub fn calculate_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction().dot(outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };
    (front_face, normal)
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
