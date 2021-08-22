mod vector;

use vector::{Color3, Point3};


const IMG_W: i32 = 256;
const IMG_H: i32 = 256;
const DYN_R: f64 = 255.999;


fn write_color(color: Color3) {
    println!(
            "{} {} {}",
            (DYN_R * color[0]) as i32,
            (DYN_R * color[1]) as i32,
            (DYN_R * color[2]) as i32
        );
}


fn main() {
    // write header for ppm image fromat
    println!("P3\n{} {}\n255", IMG_W, IMG_H);
    for j in (0..IMG_H).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMG_W {
            // norm values to be in range 0.0..1.0
            let r = i as f64 / (IMG_W - 1) as f64;
            let g = j as f64 / (IMG_H - 1) as f64;
            let b = 0.25;
            write_color(Color3::new(r, g, b));
        }
    }
}
