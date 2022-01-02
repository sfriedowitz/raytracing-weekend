use glam::DVec3;

use crate::ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left: DVec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = DVec3::new(0.0, 0.0, 0.0);
        let horizontal = DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = DVec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left =
            origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self { origin, horizontal, vertical, lower_left }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}
