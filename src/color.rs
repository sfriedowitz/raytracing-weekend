use std::fmt::Display;

use glam::DVec3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    rgb: DVec3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { rgb: DVec3::new(r, g, b) }
    }

    pub fn r(&self) -> f64 {
        self.rgb.x
    }

    pub fn g(&self) -> f64 {
        self.rgb.y
    }

    pub fn b(&self) -> f64 {
        self.rgb.z
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
            (255.999 * self.rgb.x) as u64,
            (255.999 * self.rgb.y) as u64,
            (255.999 * self.rgb.z) as u64
        )
    }
}
