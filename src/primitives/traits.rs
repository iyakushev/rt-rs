use std::ops::{
    Index, MulAssign
};

/// A main vector trait that holds 3dimensional information
pub trait Vectored
where
    Self: Sized + Index<usize, Output = f64> + MulAssign<f64>,
{
    fn new(v0: f64, v1: f64, v2: f64) -> Self;

    /// Returns a square root of the length
    fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Returns a squared length of Vec3 components
    fn len_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    /// Returns a dot product of two vectors
    fn dot(&self, other: &Self) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    /// Multiplies Vec3 components by a `value` inplace
    fn mul_by_mut(mut self, value: f64) -> Self {
        self *= value;
        self
    }

    /// Multiplies Vec3 components by a `value`
    /// and returns a new vector as a result
    fn mul_by(&self, value: f64) -> Self {
        Self::new(
            self[0] * value,
            self[1] * value,
            self[2] * value
        )
    }

    /// Returns a vector divided by it's length
    fn unit_vector(&self) -> Self {
        Self::new(
            self[0]/self.len(),
            self[1]/self.len(),
            self[2]/self.len()
        )
    }

    /// Returns a new vector which is a result of a cross-product
    fn cross(&self, other: &Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }
}

