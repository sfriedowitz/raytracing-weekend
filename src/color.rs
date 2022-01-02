use std::fmt::Display;

use glam::DVec3;

/// Type alias for vector of RGB values.
pub type RGB = DVec3;

/// Simple container for an RGB vector.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    rgb: RGB,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { rgb: DVec3::new(r, g, b) }
    }

    pub fn increment(&mut self, rgb: RGB) {
        self.rgb += rgb;
    }

    pub fn normalize(&mut self, samples: u64) {
        self.rgb = (self.rgb / samples as f64).clamp(DVec3::ZERO, DVec3::ONE);
    }
}

impl From<DVec3> for Color {
    fn from(v: DVec3) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            (256.0 * self.rgb.x) as u64,
            (256.0 * self.rgb.y) as u64,
            (256.0 * self.rgb.z) as u64
        )
    }
}
