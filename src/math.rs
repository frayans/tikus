mod vec3;

pub use vec3::{Vec3, vec3};
pub type Point3 = Vec3;

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}
