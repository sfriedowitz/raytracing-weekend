#![allow(dead_code)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod cuboid;
mod hit;
mod hittable;
mod material;
mod medium;
mod perlin;
mod ray;
mod rectangle;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec;

use material::Isotropic;
use medium::ConstantMedium;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::bvh::BVH;
use crate::camera::{Camera, ViewOptions};
use crate::color::{ray_color, Color, ColorFormat};
use crate::cuboid::Cuboid;
use crate::hittable::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::perlin::Perlin;
use crate::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use crate::rotate::RotateY;
use crate::sphere::Sphere;
use crate::texture::{ImageTexture, NoiseTexture, SolidColor};
use crate::translate::Translate;
use crate::vec::{Vec3, VecOps};

fn two_spheres() -> (HittableList, ViewOptions) {
    let texture = NoiseTexture::new(Perlin::new(), 0.5);
    let material = Lambertian::new(texture);

    let sphere1 = Sphere::stationary(Vec3::new(0.0, -10.0, 0.0), 10.0, material.clone());
    let sphere2 = Sphere::stationary(Vec3::new(0.0, 10.0, 0.0), 10.0, material);

    let mut world = HittableList::new();
    world.push(sphere1);
    world.push(sphere2);

    let opts = ViewOptions::new().with_aperture(0.1);

    (world, opts)
}

fn earth() -> (HittableList, ViewOptions) {
    let path = "${INSERT_PATH_HERE}/images/texture_earth_clouds.jpg";
    let earth_texture = ImageTexture::new(path);
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Sphere::stationary(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let mut world = HittableList::new();
    world.push(globe);

    (world, ViewOptions::new())
}

fn simple_light() -> (HittableList, ViewOptions) {
    let noise = NoiseTexture::new(Perlin::new(), 4.0);
    let material = Lambertian::new(noise);

    let sphere1 = Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material.clone());
    let sphere2 = Sphere::stationary(Vec3::new(0.0, 2.0, 0.0), 2.0, material);

    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    let rectangle = XYRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight);

    let mut world = HittableList::new();
    world.push(sphere1);
    world.push(sphere2);
    world.push(rectangle);

    let opts = ViewOptions::new()
        .with_background(Color::new(0.0, 0.0, 0.0))
        .with_lookfrom(Vec3::new(26.0, 0.0, 0.0))
        .with_lookat(Vec3::new(0.0, 2.0, 0.0))
        .with_aperture(0.1);

    (world, opts)
}

fn cornell_smoke() -> (HittableList, ViewOptions) {
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)));

    let box1 = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Translate::new(RotateY::new(box1, 15.0), Vec3::new(265.0, 0.0, 295.0));
    let box1 = ConstantMedium::new(box1, Isotropic::from(Color::new(0.0, 0.0, 0.0)), 0.01);

    let box2 = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Translate::new(RotateY::new(box2, -18.0), Vec3::new(130.0, 0.0, 65.0));
    let box2 = ConstantMedium::new(box2, Isotropic::from(Color::new(1.0, 1.0, 1.0)), 0.01);

    let mut world = HittableList::new();
    world.push(box1);
    world.push(box2);

    world.push(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white));
    world.push(XZRectangle::new(113.0, 443.0, 127.0, 432.0, 554.0, light));

    let opts = ViewOptions::new()
        .with_background(Color::new(0.0, 0.0, 0.0))
        .with_lookfrom(Vec3::new(278.0, 278.0, -800.0))
        .with_lookat(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0)
        .with_apsect_ratio(1.0)
        .with_focus_dist(10.0);

    (world, opts)
}

fn final_scene() -> (HittableList, ViewOptions) {
    let mut rng = rand::thread_rng();

    // Create ground with elevated cuboids
    let boxes_per_side = 20;
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::from(Color::new(0.48, 0.83, 0.53));

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = rng.gen_range(1.0..101.0);

            let min = Vec3::new(x0, y0, z0);
            let max = Vec3::new(x1, y1, z1);
            let cube = Cuboid::new(min, max, ground.clone());
            boxes1.push(cube);
        }
    }

    // Start adding objects to main scene
    let mut world = HittableList::new();
    world.push(BVH::new(boxes1, 0.0, 1.0));

    let light = DiffuseLight::from(Color::new(7.0, 7.0, 7.0));
    let light_rect = XZRectangle::new(123.0, 423.0, 147.0, 412.0, 554.0, light);
    world.push(light_rect);

    // Add some spheres
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere =
        Sphere::new(center1, center2, 0.0, 1.0, 50.0, Lambertian::from(Color::new(0.7, 0.3, 0.1)));
    world.push(moving_sphere);

    let dielectric_sphere =
        Sphere::stationary(Vec3::new(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5));
    let metal_sphere = Sphere::stationary(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    );
    world.push(dielectric_sphere);
    world.push(metal_sphere);

    // Boundary sphere
    let boundary1 = Sphere::stationary(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    let medium1 =
        ConstantMedium::new(boundary1.clone(), Isotropic::from(Color::new(0.2, 0.4, 0.9)), 0.2);
    world.push(boundary1);
    world.push(medium1);

    let boundary2 = Sphere::stationary(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    let medium2 =
        ConstantMedium::new(boundary2.clone(), Isotropic::from(Color::new(1.0, 1.0, 1.0)), 1e-4);
    world.push(boundary2);
    world.push(medium2);

    // Globe sphere
    let path =
        "/home/sfriedowitz/development/rust/raytracing_weekend/images/texture_earth_clouds.jpg";
    let globe_surface = Lambertian::new(ImageTexture::new(path));
    let globe = Sphere::stationary(Vec3::new(400.0, 200.0, 400.0), 100.0, globe_surface);
    world.push(globe);

    // Perlin sphere
    let perlin = Lambertian::new(NoiseTexture::new(Perlin::new(), 0.1));
    let perlin_sphere = Sphere::stationary(Vec3::new(220.0, 280.0, 300.0), 80.0, perlin);
    world.push(perlin_sphere);

    // Cube of spheres
    let ns = 1000;
    let mut boxes2 = HittableList::new();
    let white = Lambertian::from(Color::new(0.73, 0.73, 0.73));
    for _ in 0..ns {
        let sphere = Sphere::stationary(Vec3::random(0.0..165.0), 10.0, white.clone());
        boxes2.push(sphere);
    }

    let sphere_cube = BVH::new(boxes2, 0.0, 1.0);
    let sphere_cube = RotateY::new(sphere_cube, 15.0);
    let sphere_cube = Translate::new(sphere_cube, Vec3::new(-100.0, 270.0, 395.0));
    world.push(sphere_cube);

    // Image view options
    let opts = ViewOptions::new()
        .with_apsect_ratio(1.0)
        .with_image_width(800)
        .with_samples_per_pixel(1000)
        .with_background(Color::new(0.0, 0.0, 0.0))
        .with_lookfrom(Vec3::new(478.0, 278.0, -600.0))
        .with_lookat(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0);

    (world, opts)
}

fn main() {
    // World & view options
    let (world, opts) = cornell_smoke();

    // Camera
    let cam = Camera::from_options(&opts);

    println!("P3");
    println!("{} {}", opts.image_width, opts.image_height);
    println!("255");

    for j in (0..opts.image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);

        let scanline: Vec<Color> = (0..opts.image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..opts.samples_per_pixel {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((opts.image_width - 1) as f64);
                    let v = ((j as f64) + random_v) / ((opts.image_height - 1) as f64);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, opts.background, opts.max_depth);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(opts.samples_per_pixel));
        }
    }

    eprintln!("Done.");
}
