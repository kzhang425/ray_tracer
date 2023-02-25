mod vec3;
mod color;
mod ray;

use vec3::{Vec3, Color, Point3, dot};
use ray::Ray;
use std::error::Error;

// Implement a way to generate a sphere in 3d space
fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center; // A - C

    // a, b, c are basically coefficients of a quadratic equation
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;
    discriminant > 0.0
}

// Simple function to generate gradient
fn ray_color(r: &Ray) -> Color {
    let sphere_center = point3!(0, 0, -1);

    if hit_sphere(&sphere_center, 0.5, r) {
        return color_rgb!(1, 0, 0);
    }
    let unit_dir = r.direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * vector3!(1, 1, 1) + t * vector3!(0.5, 0.7, 1.0);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0; // Width : Height
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO as f64 * viewport_height;
    let focal_length = 1.0;

    let origin = point3!(0, 0, 0);
    let horizontal = vector3!(viewport_width, 0, 0);
    let vertical = vector3!(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal/2 - vertical/2 - vector3!(0, 0, focal_length);



    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255\n");
    
    // Iterate to make our first image
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;

            let ray = Ray::new(origin, direction);
            let color_pixel = ray_color(&ray);

            // Cast and write the pixel to stream
            let mut s = String::new();
            color::write_color(&mut s, color_pixel)?;
            println!("{}", s);
        }
    }

    eprintln!("\nDone\n");

    Ok(())
}
