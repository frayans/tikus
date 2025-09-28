use crate::math::Vec3;

pub type Color = Vec3;

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Vec3(r, g, b)
}
