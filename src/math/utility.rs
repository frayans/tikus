use crate::math::PI;

pub const fn deg2rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
