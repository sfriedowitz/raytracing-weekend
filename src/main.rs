mod color;
mod hit;
mod ray;
mod sphere;

use glam::DVec3;

use color::Color;
use ray::Ray;

fn hit_sphere(center: DVec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length().powi(2);
    let half_b = oc.dot(r.direction());
    let c = oc.length().powi(2) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(DVec3::new(0.0, 0.0, -1.0), 0.5, r);
    let rgb = if t > 0.0 {
        // Shade the circle based on normal direction
        let n = (r.at(t) - DVec3::new(0.0, 0.0, -1.0)).normalize();
        0.5 * DVec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        // Lerp the background based on pixel location
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0)
    };
    rgb.into()
}

fn main() -> () {
    // Image
    const ASPECT_RATIO: f64 = 16.90 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = (256.0 / ASPECT_RATIO) as u64;

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
            let color = ray_color(&r);
            println!("{}", color);
        }
    }

    eprintln!("Done.");
}
