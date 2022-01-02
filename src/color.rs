use glam::DVec3;

/// Type alias for vector of RGB values.
pub type Color = DVec3;

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
