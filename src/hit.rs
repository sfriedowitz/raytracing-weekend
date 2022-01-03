use crate::{material::Material, ray::Ray, vec::Vec3};

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub material: Material,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, material: Material) -> Self {
        Self { t, point, material, normal: Vec3::ZERO, front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
