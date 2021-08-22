use crate::vector::{Point3, Vec3};


pub struct Ray<'a> {
    pub origin: &'a Point3,
    pub direction: Vec3
}


impl<'a> Ray<'a> {
    /// Creates a new instance of the `Ray`
    pub fn new(origin: &'a Point3, direction: Vec3) -> Self {
        Ray{origin, direction}
    }

    /// Produces an arbitrary position `pos` along the `direction`
    pub fn at(&self, pos: f64) -> Point3 {
        self.origin + &self.direction.mul_by_new(pos)
    }
}
