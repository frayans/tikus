use std::f64::consts::PI;

pub const fn deg2rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
