mod vec3;
mod color;
mod ray;
mod hittable;

use vec3::{Color, Point3};
use ray::Ray;
use std::error::Error;
use hittable::*;
use hittable::sphere::Sphere;
use hittable::hittable_list::{World};

// Simple function to generate gradient
fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        // Set a background color since ray didn't hit anything
        0.5 * (rec.normal + color_rgb!(1, 1, 1))
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * color_rgb!(1, 1, 1) + t * color_rgb!(0.5, 0.7, 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0; // Width : Height
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            let color_pixel = ray_color(&ray, &world);

            // Cast and write the pixel to stream
            let mut s = String::new();
            color::write_color(&mut s, color_pixel)?;
            println!("{}", s);
        }
    }

    eprintln!("\nDone\n");

    Ok(())
}
