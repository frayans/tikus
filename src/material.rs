use rand::Rng;

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

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        if !(0.0..=1.0).contains(&fuzz) {
            panic!("`fuzz` must be in 0.0..=1.0");
        }
        Self::Metal { albedo, fuzz }
    }

    pub fn scatter<R: Rng>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatteredRay> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

                if near_zero(scatter_direction) {
                    scatter_direction = hit_record.normal;
                }

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.point, scatter_direction),
                    attenuation: *albedo,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction().reflect(hit_record.normal);
                let reflected = reflected.normalize() + (*fuzz * random_unit_vector(rng));
                let scattered = ScatteredRay {
                    ray: Ray::new(hit_record.point, reflected),
                    attenuation: *albedo,
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
    }
}
