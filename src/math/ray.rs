use super::{Number, Vec3};
pub struct Ray<T: Number> {
    origin: Vec3<T>,
    direction: Vec3<T>,
}

impl<T: Number> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::super::vec::tests::*;
    use super::*;

    type TestRay = Ray<TestPrecision>;

    #[test]
    fn at() {
        let r1 = TestRay::new(TestVec::new(0.0, 0.0, 0.0), TestVec::new(1.0, 0.0, 0.0));

        eq_helper_vec3(r1.at(3.45), TestVec::new(3.45, 0.0, 0.0));

        let r2 = TestRay::new(TestVec::new(1.0, 2.0, 3.0), TestVec::new(2.0, 1.0, 0.0));

        eq_helper_vec3(r2.at(0.5), TestVec::new(2.0, 2.5, 3.0));
    }
}
