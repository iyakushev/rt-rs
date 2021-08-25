use crate::primitives::traits::Vectored;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Type alias for vectors
pub type Vector3D = Point3D;

impl Vectored for Point3D {
    fn new(v0: f64, v1: f64, v2: f64) -> Self {
        Point3D {
            x: v0,
            y: v1,
            z: v2,
        }
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub for Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Point3D {
    type Output = Point3D;
    fn mul(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, rhs: f64) -> Self::Output {
        let reverse = 1.0 / rhs;
        self.mul_by(reverse)
    }
}

impl<'a, 'b> Add<&'b Point3D> for &'a Point3D {
    type Output = Point3D;
    fn add(self, rhs: &'b Point3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Point3D> for &'a Point3D {
    type Output = Point3D;
    fn sub(self, rhs: &'b Point3D) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, 'b> Mul<&'b Point3D> for &'a Point3D {
    type Output = Point3D;
    fn mul(self, rhs: &'b Point3D) -> Self::Output {
        Point3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> Div<f64> for &'a Point3D {
    type Output = Point3D;
    fn div(self, rhs: f64) -> Self::Output {
        let reverse = 1.0 / rhs;
        self.mul_by(reverse)
    }
}

/// Returns a new Point3D with each axis sign flipped
impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Self::Output {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Point3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Unknown index {} for Point3D", index),
        }
    }
}

impl IndexMut<usize> for Point3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Unknown index {} for Point3D", index),
        }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Point3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Point3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Point3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x *= 1.0 / rhs;
    }
}
