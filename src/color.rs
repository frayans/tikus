use crate::math::{DVec3, dvec3};

pub type Color = DVec3;

pub const fn color(r: f64, g: f64, b: f64) -> Color {
    dvec3(r, g, b)
}
