use crate::{
    color::Color,
    hittable::HitRecord,
    math::{near_zero, random_unit_vector},
    ray::Ray,
};

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = hit_record.normal + random_unit_vector(&mut rand::rng());

        if near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    /// must be in range `0.0..1.0`
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = ray_in.direction().reflect(hit_record.normal);
        let reflected = reflected.normalize() + (self.fuzz * random_unit_vector(&mut rand::rng()));
        let scattered = ScatteredRay {
            ray: Ray::new(hit_record.point, reflected),
            attenuation: self.albedo,
        };
        if scattered
            .ray
            .direction()
            .dot(hit_record.normal)
            .is_sign_positive()
        {
            Some(scattered)
        } else {
            None
        }
    }
}
