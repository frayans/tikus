use rand::Rng;

use crate::{
    color::{Color, color},
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
    Dielectric { refraction_index: f64 },
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo }
    }

    /// fuzz must be in 0.0..=1.0
    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(refraction_index: f64) -> Self {
        Self::Dielectric { refraction_index }
    }

    pub fn scatter<R: Rng>(
        &self,
        rng: &mut R,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatteredRay> {
        match *self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

                if near_zero(scatter_direction) {
                    scatter_direction = hit_record.normal;
                }

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.point, scatter_direction),
                    attenuation: albedo,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction().reflect(hit_record.normal);
                let reflected = reflected.normalize() + (fuzz * random_unit_vector(rng));
                let scattered = ScatteredRay {
                    ray: Ray::new(hit_record.point, reflected),
                    attenuation: albedo,
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
            Material::Dielectric { refraction_index } => {
                let attenuation = color(1.0, 1.0, 1.0);
                let ri = if hit_record.front_face {
                    1.0 / refraction_index
                } else {
                    refraction_index
                };

                let unit_dir = ray_in.direction().normalize();
                let cos_theta = (-unit_dir.dot(hit_record.normal)).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let random_double = rng.random_range(0.0..1.0);
                let cannot_refract = (ri * sin_theta) > 1.0;
                let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_double
                {
                    unit_dir.reflect(hit_record.normal)
                } else {
                    unit_dir.refract(hit_record.normal, ri)
                };

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.point, direction),
                    attenuation,
                })
            }
        }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
