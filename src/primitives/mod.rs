pub mod traits;
pub mod objects;
pub mod color;
pub mod ray;

use std::f64::consts::PI;


#[inline(always)]
pub fn degrees_to_radiance(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
