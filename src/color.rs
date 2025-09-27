use std::io;
use std::io::Write;

use crate::math::{Vec3, vec3};

pub type Color = Vec3;

pub fn color(r: f64, g: f64, b: f64) -> Color {
    vec3(r, g, b)
}

impl Color {
    pub fn write_to<W: Write>(&self, mut w: W) -> io::Result<()> {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let ir = (255.999 * r) as isize;
        let ig = (255.999 * g) as isize;
        let ib = (255.999 * b) as isize;

        writeln!(w, "{} {} {}", ir, ig, ib)?;
        Ok(())
    }
}
