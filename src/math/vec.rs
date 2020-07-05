use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Number:
    Copy
    + Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + Sqrt
{
}

impl Number for f32 {}
impl Number for f64 {}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<f32>;

impl<T: Number> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> T {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Number> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Number> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: Number> Sub for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Number> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<T: Number> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Number> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<T: Number> Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Number> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<T: Number> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON_F: f32 = 0.00000001;

    fn eq_helper(lhs: Vec3f, rhs: Vec3f) {
        assert!((lhs.x - rhs.x).abs() < EPSILON_F);
        assert!((lhs.y - rhs.y).abs() < EPSILON_F);
        assert!((lhs.z - rhs.z).abs() < EPSILON_F);
    }

    #[test]
    fn add() {
        let v1 = Vec3f::new(1.23, 4.56, 7.89);
        let v2 = Vec3f::new(6.54, 3.12, 8.97);

        let expected = Vec3f::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);

        let result = v1 + v2;
        eq_helper(result, expected);
    }

    #[test]
    fn sub() {
        let v1 = Vec3f::new(1.23, 4.56, 7.89);
        let v2 = Vec3f::new(6.54, 3.12, 8.97);

        let expected = Vec3f::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);

        let result = v1 - v2;
        eq_helper(result, expected);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3f::new(1.23, 4.56, 7.89);
        let v2 = Vec3f::new(6.54, 3.12, 8.97);

        let expected = Vec3f::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);

        v1 += v2;
        eq_helper(v1, expected);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vec3f::new(1.23, 4.56, 7.89);
        let v2 = Vec3f::new(6.54, 3.12, 8.97);

        let expected = Vec3f::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);

        v1 -= v2;
        eq_helper(v1, expected);
    }

    #[test]
    fn mul() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = Vec3f::new(v1.x * s1, v1.y * s1, v1.z * s1);

        let result = v1 * s1;
        eq_helper(result, expected);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3f::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = Vec3f::new(v1.x * s1, v1.y * s1, v1.z * s1);

        v1 *= s1;
        eq_helper(v1, expected);
    }

    #[test]
    fn div() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = Vec3f::new(v1.x / s1, v1.y / s1, v1.z / s1);

        let result = v1 / s1;
        eq_helper(result, expected);
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vec3f::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = Vec3f::new(v1.x / s1, v1.y / s1, v1.z / s1);

        v1 /= s1;
        eq_helper(v1, expected);
    }
}
