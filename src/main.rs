mod ray;
mod vector;

use ray::Ray;
use vector::{Color3, Point3};

use crate::vector::Vec3;

const DYN_R: f64 = 255.999;
// IMG
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_W: i32 = 400;
const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;
// Camera
const VIEWPORT_H: f64 = 2.0;
const VIEWPORT_W: f64 = ASPECT_RATIO * VIEWPORT_H;
const FOCAL_LEN: f64 = 1.0;

fn ray_color(ray: &Ray) -> Color3 {
    let direction = ray.direction.unit_vector();
    let pos = 0.5 * (direction.y + 1.0);
    &*Color3::new(1.0, 1.0, 1.0).mul_by(1.0 - pos) + &*Color3::new(0.5, 0.7, 1.0).mul_by(pos)
}

fn write_color(color: Color3) {
    println!(
        "{} {} {}",
        (DYN_R * color[0]) as i32,
        (DYN_R * color[1]) as i32,
        (DYN_R * color[2]) as i32
    );
}

fn main() {
    // camera setup
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_W, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_H, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LEN);

    // write header for ppm image fromat
    println!("P3\n{} {}\n255", IMG_W, IMG_H);
    for j in (0..IMG_H).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMG_W {
            // norm values to be in range 0.0..1.0
            let u = i as f64 / (IMG_W - 1) as f64;
            let v = j as f64 / (IMG_H - 1) as f64;
            let ray = Ray::new(
                &origin,
                &lower_left_corner + &horizontal.mul_by_new(u) + (&vertical.mul_by_new(v) - &origin),
            );
            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
}
