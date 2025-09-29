use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.end;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, ray_t.start..closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
