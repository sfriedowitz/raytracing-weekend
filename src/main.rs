mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod hittable;
mod material;
mod perlin;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod vec;

use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use texture::{CheckerTexture, NoiseTexture};

use crate::camera::Camera;
use crate::color::{ray_color, Color, ColorFormat};
use crate::hittable::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::perlin::Perlin;
use crate::rectangle::Rectangle;
use crate::sphere::Sphere;
use crate::texture::{ImageTexture, SolidColor};
use crate::vec::{Vec3, VecOps};

fn two_spheres() -> HittableList {
    let texture = NoiseTexture::new(Perlin::new(), 0.5);
    let sphere1 = Sphere::stationary(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(texture.clone().into()).into(),
    );
    let sphere2 =
        Sphere::stationary(Vec3::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(texture.into()).into());

    vec![sphere1.into(), sphere2.into()]
}

fn earth() -> HittableList {
    let path =
        "/home/sfriedowitz/development/rust/raytracing_weekend/images/texture_earth_clouds.jpg";
    let earth_texture = ImageTexture::new(path);
    let earth_surface = Lambertian::new(earth_texture.into());
    let globe = Sphere::stationary(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface.into());
    vec![globe.into()]
}

fn simple_light() -> HittableList {
    let noise = NoiseTexture::new(Perlin::new(), 4.0);
    let material = Lambertian::new(noise.into());

    let sphere1 = Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material.clone().into());
    let sphere2 = Sphere::stationary(Vec3::new(0.0, 2.0, 0.0), 2.0, material.into());

    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)).into());
    let rectangle = Rectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.into());

    vec![sphere1.into(), sphere2.into(), rectangle.into()]
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut objects = HittableList::new();

    let checker = CheckerTexture::from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_mat = Lambertian::new(checker.into());
    let ground_sphere = Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat.into());

    objects.push(ground_sphere.into());

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Lambertian::from_color(albedo);
                let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                let moving_sphere = Sphere::new(center, center2, 0.0, 1.0, 0.2, sphere_mat.into());
                objects.push(moving_sphere.into());
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Metal::new(albedo, fuzz);
                let sphere = Sphere::stationary(center, 0.2, sphere_mat.into());

                objects.push(sphere.into());
            } else {
                // Glass
                let sphere_mat = Dielectric::new(1.5);
                let sphere = Sphere::stationary(center, 0.2, sphere_mat.into());

                objects.push(sphere.into());
            }
        }
    }

    let matime1 = Dielectric::new(1.5);
    let mat2 = Lambertian::from_color(Color::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::stationary(Vec3::new(0.0, 1.0, 0.0), 1.0, matime1.into());
    let sphere2 = Sphere::stationary(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2.into());
    let sphere3 = Sphere::stationary(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3.into());

    objects.push(sphere1.into());
    objects.push(sphere2.into());
    objects.push(sphere3.into());

    objects
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1024;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 400;
    const MAX_DEPTH: u64 = 50;

    // World
    let world = simple_light();
    let background = Color::new(0.0, 0.0, 0.0);

    // Camera
    let lookfrom = Vec3::new(26.0, 3.0, 6.0);
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, vfov, 0.0, 1.0);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, background, MAX_DEPTH);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
