use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self { origin, direction, time }
    }

    /// Get the origin of this ray.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// Get the direction of travel of this ray.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Get the time of origination of this ray.
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Get the ray position at contour position `s`.
    pub fn at(&self, s: f64) -> Vec3 {
        self.origin + s * self.direction
    }
}
