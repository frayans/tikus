pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub const fn deg2rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
