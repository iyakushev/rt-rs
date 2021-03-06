use super::{objects::HitRecord, rand_f64, rand_in_range, ray::Ray};
use std::ops::{Index, IndexMut, MulAssign, Range};



/// This trait defines an interface for any object that can be hit by a ray
pub trait Solid {
    fn hit(&self, ray: &Ray, pos_min: f64, pos_max: f64) -> Option<HitRecord>;
}


/// A main vector trait that performs 3d calculations on Vectored objects
pub trait Vectored
where
    Self: Sized + Index<usize, Output = f64> + IndexMut<usize> + MulAssign<f64>,
{
    /// Creates a new instance of the 3-dimensional Vectored object
    fn new(v0: f64, v1: f64, v2: f64) -> Self;

    /// Creates a new instance of the 3-dimensional Vectored object with random coordinates
    fn random() -> Self {
        Self::new(rand_f64(), rand_f64(), rand_f64())
    }

    /// Creates a new instance of the 3-dimensional Vectored object with random coordinates in
    /// specified range
    fn random_in(range: Range<f64>) -> Self {
        Self::new(rand_in_range(&range), rand_in_range(&range), rand_in_range(&range))
    }

    /// Implements hemispherical scattering diffusion
    fn random_in_hemisphere(normal: &impl Vectored) -> Self {
        let in_sphere = Self::random_in_unit_sphere();
        let mask: i32 = in_sphere.dot(normal) as i32 >> 31;
        in_sphere.mul_by_mut((-1.0f64).powi(mask))
    }

    fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector_mut()
    }

    /// used for Lambertian reflections
    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(-1.0..1.0);
            if p.len_squared() >= 1.0 {
                continue
            }
            return p
        }
    }

    /// Returns a square root of the length
    fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Returns a squared length of Vec3 components
    fn len_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    /// Returns a dot product of two vectors
    fn dot(&self, other: &impl Vectored) -> f64 {
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
        Self::new(self[0] * value, self[1] * value, self[2] * value)
    }

    /// Returns a new vector divided by it's length
    fn unit_vector(&self) -> Self {
        let len = self.len();
        Self::new(
            self[0] / len,
            self[1] / len,
            self[2] / len,
        )
    }

    /// Returns a unit vector transformed the self
    fn unit_vector_mut(mut self) -> Self {
        let len = self.len();
        self[0] /= len;
        self[1] /= len;
        self[2] /= len;
        self
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
