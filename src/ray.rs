use crate::math::{DVec3, Point3};

#[derive(Debug, Default)]
pub struct Ray {
    orig: Point3,
    dir: DVec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: DVec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub const fn origin(&self) -> Point3 {
        self.orig
    }

    pub const fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
