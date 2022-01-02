mod camera;
mod color;
mod hit;
mod hittable;
mod ray;
mod sphere;

use glam::DVec3;

use color::{Color, RGB};
use hit::Hit;
use hittable::{Hittable, World};
use ray::Ray;

fn ray_color(r: &Ray, world: &World) -> Color {
    let rgb = if let Some(record) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (record.normal + RGB::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * RGB::new(1.0, 1.0, 1.0) + t * RGB::new(0.5, 0.7, 1.0)
    };

    rgb.into()
}

fn main() -> () {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = (256.0 / ASPECT_RATIO) as u64;

    // World
    let mut world = World::new();
    world.push(Hittable::sphere(0.5, DVec3::new(0.0, 0.0, -1.0)));
    world.push(Hittable::sphere(100.0, DVec3::new(0.0, -100.5, -1.0)));

    // Camera
    let viewort_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewort_height;
    let focal_length = 1.0;

    let origin = DVec3::new(0.0, 0.0, 0.0);
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewort_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - j - 1);

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r, &world);

            println!("{}", pixel_color);
        }
    }

    eprintln!("Done.");
}
