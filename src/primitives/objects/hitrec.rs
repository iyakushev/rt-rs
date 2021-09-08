use crate::primitives::{material::Material, ray::Ray, traits::Vectored};

use super::{Point3D, Vector3D};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vector3D,
    pub pos: f64,
}

impl HitRecord {
    pub fn set_face_norm(&mut self, ray: &Ray, outward_norm: Vector3D) {
        let front_face = ray.direction.dot(&outward_norm) as i32 >> 31;
        self.normal = outward_norm.mul_by_mut((-1.0f64).powi(front_face));
    }
}
