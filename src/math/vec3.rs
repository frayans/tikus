use std::ops::{self, Range};

use rand::Rng;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Vec3(rng.random(), rng.random(), rng.random())
    }

    pub fn random_range<R: Rng>(rng: &mut R, range: Range<f64>) -> Self {
        let start = range.start;
        let end = range.end;
        Vec3(
            rng.random_range(start..end),
            rng.random_range(start..end),
            rng.random_range(start..end),
        )
    }

    pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Self {
        loop {
            let p = Self::random_range(rng, -1.0..1.0);
            let lensq = p.mag2();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere<R: Rng>(rng: &mut R, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if on_unit_sphere.dot(*normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    #[doc(alias = "length")]
    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }

    #[doc(alias = "length_squared")]
    pub fn mag2(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[doc(alias = "unit_vector")]
    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self / rhs.x(), self / rhs.y(), self / rhs.z())
    }
}
