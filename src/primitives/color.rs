use super::traits::Vectored;
use std::ops::{Add, AddAssign, Index, IndexMut, MulAssign};

#[derive(Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Vectored for Color {
    fn new(v0: f64, v1: f64, v2: f64) -> Self {
        Color {
            r: v0,
            g: v1,
            b: v2,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T> Add<T> for Color
where
    T: Vectored,
{
    type Output = Color;
    fn add(self, rhs: T) -> Self::Output {
        Color::new(self.r + rhs[0], self.g + rhs[1], self.b + rhs[2])
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Unknown index {} for Point3D", index),
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Unknown index {} for Point3D", index),
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}
