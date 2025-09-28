use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<H: Hittable + 'a>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList<'_> {
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
