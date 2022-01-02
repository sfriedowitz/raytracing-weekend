mod color;
mod ray;

use glam::DVec3;

use color::Color;
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let rgb = (1.0 - t) * Color::new(1.0, 1.0, 1.0).rgb + t * Color::new(0.5, 0.7, 1.0).rgb;
    Color::new(rgb.x, rgb.y, rgb.z)
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
