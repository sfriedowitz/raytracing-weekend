#![allow(dead_code)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod cuboid;
mod hit;
mod hittable;
mod material;
mod perlin;
mod ray;
mod rectangle;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec;

use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::camera::{Camera, ViewOptions};
use crate::color::{ray_color, Color, ColorFormat};
use crate::cuboid::Cuboid;
use crate::hittable::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::perlin::Perlin;
use crate::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use crate::rotate::RotateY;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::translate::Translate;
use crate::vec::{Vec3, VecOps};

fn two_spheres() -> (HittableList, ViewOptions) {
    let texture = NoiseTexture::new(Perlin::new(), 0.5);
    let material = Lambertian::new(texture.into());

    let sphere1 = Sphere::stationary(Vec3::new(0.0, -10.0, 0.0), 10.0, material.clone().into());
    let sphere2 = Sphere::stationary(Vec3::new(0.0, 10.0, 0.0), 10.0, material.into());

    let world = vec![sphere1.into(), sphere2.into()];
    let opts = ViewOptions::new().with_aperture(0.1);

    (world, opts)
}

fn earth() -> (HittableList, ViewOptions) {
    let path = "${INSERT_PATH_HERE}/images/texture_earth_clouds.jpg";
    let earth_texture = ImageTexture::new(path);
    let earth_surface = Lambertian::new(earth_texture.into());
    let globe = Sphere::stationary(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface.into());

    let world = vec![globe.into()];
    (world, ViewOptions::new())
}

fn simple_light() -> (HittableList, ViewOptions) {
    let noise = NoiseTexture::new(Perlin::new(), 4.0);
    let material = Lambertian::new(noise.into());

    let sphere1 = Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material.clone().into());
    let sphere2 = Sphere::stationary(Vec3::new(0.0, 2.0, 0.0), 2.0, material.into());

    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)).into());
    let rectangle = XYRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.into());
    let world = vec![sphere1.into(), sphere2.into(), rectangle.into()];

    let opts = ViewOptions::new()
        .with_background(Color::new(0.0, 0.0, 0.0))
        .with_lookfrom(Vec3::new(26.0, 0.0, 0.0))
        .with_lookat(Vec3::new(0.0, 2.0, 0.0))
        .with_aperture(0.1);

    (world, opts)
}

fn cornell_box() -> (HittableList, ViewOptions) {
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)).into());
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)).into());
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)).into());
    let light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)).into());

    let box1 =
        Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone().into());
    let box2 =
        Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone().into());

    let world: HittableList = vec![
        YZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green.into()).into(),
        YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red.into()).into(),
        XZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone().into()).into(),
        XZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone().into()).into(),
        XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone().into()).into(),
        XZRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light.into()).into(),
        Translate::new(
            Box::new(RotateY::new(Box::new(box1.into()), 15.0).into()),
            //Box::new(box1.into()),
            Vec3::new(265.0, 0.0, 295.0),
        )
        .into(),
        Translate::new(
            Box::new(RotateY::new(Box::new(box2.into()), -18.0).into()),
            //Box::new(box2.into()),
            Vec3::new(130.0, 0.0, 65.0),
        )
        .into(),
    ];

    let opts = ViewOptions::new()
        .with_background(Color::new(0.0, 0.0, 0.0))
        .with_lookfrom(Vec3::new(278.0, 278.0, -800.0))
        .with_lookat(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0)
        .with_apsect_ratio(1.0)
        .with_focus_dist(10.0);

    (world, opts)
}

fn random_scene() -> (HittableList, ViewOptions) {
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

    (objects, ViewOptions::new())
}

fn main() {
    // World & view options
    let (world, opts) = cornell_box();

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
