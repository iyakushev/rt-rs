mod primitives;
use primitives::camera::Camera;
use primitives::color::Color;
use primitives::objects::{Point3D, Vector3D};
use primitives::ray::Ray;
use primitives::traits::{HitRecord, Solid, Vectored};

use crate::primitives::objects::{CollisionList, Sphere};
use crate::primitives::rand_f64;

// IMG
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_W: i32 = 400;
const IMG_H: i32 = (IMG_W as f64 / ASPECT_RATIO) as i32;

const SAMPLES_PER_PIXEL: i32 = 100;
const ALIASING_SCALE: f64 = 1.0/SAMPLES_PER_PIXEL as f64;
const MAX_TRACE_DEPTH: u32 = 50;

fn ray_color(ray: Ray, world: &dyn Solid, depth: u32) -> Color {
    let mut record: HitRecord = Default::default();
    if depth == 0 {
        return Color::default()
    }
    if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        let target = &record.point + &record.normal + Vector3D::random_unit_vector();
        return ray_color(Ray::new(&record.point, &target - &record.point), world, depth - 1).mul_by(0.5);
    }
    let direction = ray.direction.unit_vector();
    let pos = 0.5 * (direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0).mul_by(1.0 - pos) + Color::new(0.5, 0.7, 1.0).mul_by(pos)
}

fn write_color(color: &Color) {
    let color = color.mul_by(ALIASING_SCALE).gamma_correction(2.0);
    println!(
        "{} {} {}",
        (256.0 * color[0].clamp(0.0, 0.999)) as i32,
        (256.0 * color[1].clamp(0.0, 0.999)) as i32,
        (256.0 * color[2].clamp(0.0, 0.999)) as i32
    );
}

fn main() {
    let camera = Camera::new();
    // world
    let mut world = CollisionList::new();
    world.push(Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // write header for ppm image fromat
    println!("P3\n{} {}\n255", IMG_W, IMG_H);
    for j in (0..IMG_H).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMG_W {
            let mut pixel_color = Color::default();
            // antialiasing
            for _ in 0..SAMPLES_PER_PIXEL {
                // norm values to be in range 0.0..1.0 + add random divergence for aliasing
                let u = (i as f64 + rand_f64()) / (IMG_W - 1) as f64;
                let v = (j as f64 + rand_f64()) / (IMG_H - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &world, MAX_TRACE_DEPTH);
            }
            write_color(&pixel_color);
        }
    }
}
