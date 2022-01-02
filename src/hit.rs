use glam::DVec3;

use crate::material::Material;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: DVec3,
    pub material: Material,
    pub normal: DVec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: DVec3, material: Material) -> Self {
        Self { t, point, material, normal: DVec3::ZERO, front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
