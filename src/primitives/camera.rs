use super::{
    objects::{Point3D, Vector3D},
    ray::Ray,
    traits::Vectored,
};

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vector3D,
    vertical: Vector3D,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_h = 2.0;
        let viewport_w = aspect_ratio * viewport_h;
        let focal_length = 1.0;

        let origin = Point3D::default();
        let horizontal = Vector3D {
            x: viewport_w,
            ..Default::default()
        };
        let vertical = Vector3D::new(0.0, viewport_h, 0.0);
        let lower_left_corner = &origin
            - &(&horizontal / 2.0)
            - &vertical / 2.0
            - Vector3D::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    /// Returns a ray by the given u,v coordinates
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &self.lower_left_corner
                + &self.horizontal.mul_by(u)
                + (&self.vertical.mul_by(v) - &self.origin),
        )
    }
}
