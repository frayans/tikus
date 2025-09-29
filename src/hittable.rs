use std::ops::Range;

use crate::{
    material::Material,
    math::{DVec3, Point3},
    ray::Ray,
};

pub struct HitRecord<'a> {
    pub point: Point3,
    pub t: f64,
    pub normal: DVec3,
    pub mat: Box<dyn Material + 'a>,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn with_face_normal<M: Material + Copy + 'a>(
        point: Point3,
        t: f64,
        mat: &M,
        ray: &Ray,
        outward_normal: DVec3,
    ) -> Self {
        let (front_face, normal) = Self::calculate_face_normal(ray, outward_normal);
        HitRecord {
            point,
            t,
            normal,
            mat: Box::new(*mat),

            front_face,
        }
    }

    fn calculate_face_normal(ray: &Ray, outward_normal: DVec3) -> (bool, DVec3) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord<'_>>;
}
