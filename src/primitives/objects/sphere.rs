use super::Point3D;
use crate::primitives::traits::{HitRecord, Solid, Vectored};

pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Solid for Sphere {
    fn hit(
        &self,
        ray: &crate::primitives::ray::Ray,
        pos_min: f64,
        pos_max: f64,
        record: &mut HitRecord,
    ) -> bool {
        let origin = ray.origin - &self.center;
        let a = ray.direction.len_squared();
        let b_halfed = origin.dot(&ray.direction);
        let c = origin.len_squared() - self.radius * self.radius;
        let discriminant = b_halfed * b_halfed - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        // finding nearest root
        let mut root = (-b_halfed - sqrt_d) / a;
        if root < pos_min || pos_max < root {
            root = (-b_halfed + sqrt_d) / a;
            if root < pos_min || pos_max < root {
                return false;
            }
        }

        record.pos = root;
        record.point = ray.at(root);
        let outward_norm = (&record.point - &self.center) / self.radius;
        record.set_face_norm(ray, outward_norm);
        true
    }
}
