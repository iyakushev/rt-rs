pub mod camera;
pub mod color;
pub mod objects;
pub mod ray;
pub mod traits;

use rand::prelude::*;
use std::{f64::consts::PI, ops::Range};

/// Returns random value in the rango from [0, 1)
#[inline(always)]
pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Generates a random number in the given range
#[inline(always)]
pub fn rand_in_range(range: &Range<f64>) -> f64 {
    range.start + (range.end - range.start) * rand_f64()
}

#[inline(always)]
pub fn _degrees_to_radiance(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
