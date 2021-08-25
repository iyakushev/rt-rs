use super::objects::{Point3D, Vector3D};
use super::traits::Vectored;


pub struct Ray<'a> {
    pub origin: &'a Point3D,
    pub direction: Vector3D
}


impl<'a> Ray<'a> {
    /// Creates a new instance of the `Ray`
    pub fn new(origin: &'a Point3D, direction: Vector3D) -> Self {
        Ray{origin, direction}
    }

    /// Produces an arbitrary position `pos` along the `direction`
    pub fn at(&self, pos: f64) -> Point3D {
        self.origin + &self.direction.mul_by(pos)
    }
}
