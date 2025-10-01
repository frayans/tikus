mod utility;
// mod vec3;

use std::ops::Range;

pub use glam::{DVec3, dvec3};
use rand::Rng;
pub use utility::deg2rad;
pub type Point3 = glam::DVec3;

pub const fn point3(x: f64, y: f64, z: f64) -> Point3 {
    dvec3(x, y, z)
}

pub fn random<R: Rng>(rng: &mut R) -> DVec3 {
    dvec3(rng.random(), rng.random(), rng.random())
}

pub fn random_range<R: Rng>(rng: &mut R, range: Range<f64>) -> DVec3 {
    let start = range.start;
    let end = range.end;
    dvec3(
        rng.random_range(start..end),
        rng.random_range(start..end),
        rng.random_range(start..end),
    )
}

pub fn random_unit_vector<R: Rng>(rng: &mut R) -> DVec3 {
    loop {
        let p = random_range(rng, -1.0..1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere<R: Rng>(rng: &mut R, normal: &DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector(rng);
    if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk<R: Rng>(rng: &mut R) -> DVec3 {
    loop {
        let p = dvec3(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0), 0.);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(v: DVec3) -> bool {
    const S: f64 = 1e-8;
    (v.x.abs() < S) && (v.y.abs() < S) && (v.z.abs() < S)
}
