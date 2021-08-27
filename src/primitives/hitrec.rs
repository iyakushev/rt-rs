use super::{objects::{Point3D, Vector3D}, ray::Ray, traits::Vectored};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vector3D,
    pub front_face: bool,
    pub pos: f64,
}

impl HitRecord {
    pub fn set_face_norm(&mut self, ray: &Ray, outward_norm: Vector3D) {
        self.front_face = ray.direction.dot(&outward_norm) as i32 >> 31;

        self.normal = if self.front_face {
            outward_norm
        } else {
            -outward_norm
        }
    }
}
