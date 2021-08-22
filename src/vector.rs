use core::panic;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// A main vector struct that holds 3dimensional information
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

// create handy type aliases
pub type Point3 = Vec3;
pub type Color3 = Vec3;


// TODO add later q_rsqrt?
impl Vec3 {
    pub fn new(v0: f64, v1: f64, v2: f64) -> Self {
        Vec3 {
            x: v0,
            y: v1,
            z: v2,
        }
    }

    /// Returns a square root of the length
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Returns a squared length of Vec3 components
    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Returns a dot product of two vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Multiplies Vec3 components by a `value` inplace
    pub fn mul_by(&mut self, value: f64) {
        *self *= value;
    }

    /// Multiplies Vec3 components by a `value`
    /// and returns a new vector as a result
    fn mul_by_new(&self, value: f64) -> Vec3 {
        Vec3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }

    /// Returns a vector divided by it's length
    pub fn unit_vector(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.len(),
            y: self.y / self.len(),
            z: self.z / self.len(),
        }
    }

    /// Returns a new vector which is a result of a cross-product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let reverse = 1.0 / rhs;
        self.mul_by_new(reverse)
    }
}

/// Returns a new Vec3 with each axis sign flipped
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Unknown index {} for Vec3", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Unknown index {} for Vec3", index),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x *= 1.0 / rhs;
    }
}
