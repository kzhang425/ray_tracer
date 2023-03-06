mod vec3;
mod color;
mod ray;
mod hittable;
mod camera;

use vec3::{Color, Point3};
use ray::Ray;
use std::error::Error;
use hittable::*;
use hittable::sphere::Sphere;
use hittable::hittable_list::{World};
use camera::Camera;
use rand;
use rand::Rng;

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
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();


    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255\n");
    
    let mut rng = rand::thread_rng();
    // Iterate to make our first image
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {

            // Make a a pixel color, decide whether it is an edge
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // Essentially averages edges so that they look softer. There has to be a more efficient way to do this!
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("\nDone\n");

    Ok(())
}
