mod camera;
mod color;
mod hit;
mod hittable;
mod material;
mod ray;
mod vec;

use glam::DVec3;
use material::Scatter;
use rand::{thread_rng, Rng};

use crate::camera::Camera;
use crate::color::{Color, ColorFormat};
use crate::hit::Hit;
use crate::hittable::{Hittable, Sphere, World};
use crate::material::{Lambertian, Material, Metal};
use crate::ray::Ray;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() -> () {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 10;

    // World
    let mut world = World::new();
    let mat_ground: Material = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let mat_center: Material = Lambertian::new(Color::new(0.7, 0.3, 0.3)).into();
    let mat_left: Material = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3).into();
    let mat_right: Material = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    let sphere_ground: Hittable =
        Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0, mat_ground).into();
    let sphere_center: Hittable = Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5, mat_center).into();
    let sphere_left: Hittable = Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, mat_left).into();
    let sphere_right: Hittable = Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, mat_right).into();

    world.push(sphere_ground);
    world.push(sphere_center);
    world.push(sphere_left);
    world.push(sphere_right);

    // Camera
    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
