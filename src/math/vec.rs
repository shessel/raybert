use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::{Display, Debug};
use std::cmp::{PartialOrd};

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
pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
pub trait Epsilon {
    const EPSILON: Self;
}

impl Epsilon for f32 {
    const EPSILON: Self = f32::EPSILON;
}

impl Epsilon for f64 {
    const EPSILON: Self = f64::EPSILON;
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
    + Abs
    + Epsilon
    + Display
    + Debug
    + PartialOrd
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

    pub fn normalize(&mut self) {
        *self /= self.len()
    }
}

pub fn dot<T: Number>(lhs: Vec3<T>, rhs: Vec3<T>) -> T {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
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
    type TestPrecision = f32;
    type TestVec = Vec3<TestPrecision>;

    fn eq_helper_vec3<T: Number>(lhs: Vec3<T>, rhs: Vec3<T>) {
        assert!((lhs.x - rhs.x).abs() < T::EPSILON, "{} != {}", lhs.x, rhs.x);
        assert!((lhs.y - rhs.y).abs() < T::EPSILON, "{} != {}", lhs.y, rhs.y);
        assert!((lhs.z - rhs.z).abs() < T::EPSILON, "{} != {}", lhs.z, rhs.z);
    }

    fn eq_helper<T: Number>(lhs: T, rhs: T) {
        assert!((lhs - rhs).abs() < T::EPSILON, "{} != {}", lhs, rhs);
    }

    #[test]
    fn add() {
        let v1 = TestVec::new(1.23, 4.56, 7.89);
        let v2 = TestVec::new(6.54, 3.12, 8.97);

        let expected = TestVec::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);

        let result = v1 + v2;
        eq_helper_vec3(result, expected);
    }

    #[test]
    fn sub() {
        let v1 = TestVec::new(1.23, 4.56, 7.89);
        let v2 = TestVec::new(6.54, 3.12, 8.97);

        let expected = TestVec::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);

        let result = v1 - v2;
        eq_helper_vec3(result, expected);
    }

    #[test]
    fn add_assign() {
        let mut v1 = TestVec::new(1.23, 4.56, 7.89);
        let v2 = TestVec::new(6.54, 3.12, 8.97);

        let expected = TestVec::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);

        v1 += v2;
        eq_helper_vec3(v1, expected);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = TestVec::new(1.23, 4.56, 7.89);
        let v2 = TestVec::new(6.54, 3.12, 8.97);

        let expected = TestVec::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);

        v1 -= v2;
        eq_helper_vec3(v1, expected);
    }

    #[test]
    fn mul() {
        let v1 = TestVec::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = TestVec::new(v1.x * s1, v1.y * s1, v1.z * s1);

        let result = v1 * s1;
        eq_helper_vec3(result, expected);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = TestVec::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = TestVec::new(v1.x * s1, v1.y * s1, v1.z * s1);

        v1 *= s1;
        eq_helper_vec3(v1, expected);
    }

    #[test]
    fn div() {
        let v1 = TestVec::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = TestVec::new(v1.x / s1, v1.y / s1, v1.z / s1);

        let result = v1 / s1;
        eq_helper_vec3(result, expected);
    }

    #[test]
    fn div_assign() {
        let mut v1 = TestVec::new(1.0, 2.0, 3.0);
        let s1 = 2.0;

        let expected = TestVec::new(v1.x / s1, v1.y / s1, v1.z / s1);

        v1 /= s1;
        eq_helper_vec3(v1, expected);
    }

    #[test]
    fn len() {
        let v1 = TestVec::new(1.0, 2.0, 3.0);
        let expected = (v1.x * v1.x + v1.y * v1.y + v1.z * v1.z).sqrt();
        eq_helper(v1.len(), expected);

        let v2 = TestVec::new(5.0, 0.0, 0.0);
        eq_helper(v2.len(), 5.0);

        let v3 = TestVec::new(1.0, 1.0, 0.0);
        eq_helper(v3.len(), 2.0.sqrt());
    }

    #[test]
    fn len_squared() {
        let v1 = TestVec::new(1.0, 2.0, 3.0);
        let expected = v1.x * v1.x + v1.y * v1.y + v1.z * v1.z;
        eq_helper(v1.len_squared(), expected);

        let v2 = TestVec::new(5.0, 0.0, 0.0);
        eq_helper(v2.len_squared(), 25.0);

        let v3 = TestVec::new(1.0, 1.0, 0.0);
        eq_helper(v3.len_squared(), 4.0.sqrt());
    }

    #[test]
    fn normalize() {
        let mut v1 = TestVec::new(5.0, 0.0, 0.0);
        v1.normalize();

        let expected = TestVec::new(1.0, 0.0, 0.0);
        eq_helper_vec3(v1, expected);

        let mut v2 = TestVec::new(5.0, 5.0, 0.0);
        v2.normalize();
        
        let expected2 = TestVec::new(0.5.sqrt(), 0.5.sqrt(), 0.0);
        eq_helper_vec3(v2, expected2);
    }

    #[test]
    fn dot() {
        let v1 = TestVec::new(5.0, 0.0, 0.0);
        let v2 = TestVec::new(0.0, 4.0, 0.0);
        eq_helper(super::dot(v1, v2), 0.0);
        eq_helper(super::dot(v1, v1), v1.len_squared());
    }
}
