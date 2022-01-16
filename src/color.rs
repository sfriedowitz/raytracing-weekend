use crate::{hit::Hit, hittable::HittableList, material::Scatter, ray::Ray, vec::Vec3};

/// Type alias for vector of RGB values.
pub type Color = Vec3;

pub trait ColorFormat {
    fn format_color(&self, num_samples: u64) -> String;
}

impl ColorFormat for Color {
    fn format_color(&self, num_samples: u64) -> String {
        let ir = (256.0 * (self.x / (num_samples as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.y / (num_samples as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.z / (num_samples as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }
}

pub fn ray_color(r: &Ray, world: &HittableList, background: Color, depth: u64) -> Color {
    if depth == 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.material.emitted(rec.point, rec.u, rec.v);
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, world, background, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}
