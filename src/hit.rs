use crate::{aabb::AABB, material::Material, ray::Ray, vec::Vec3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub s: f64,
    pub u: f64,
    pub v: f64,
    pub point: Vec3,
    pub material: Material,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(s: f64, u: f64, v: f64, point: Vec3, material: Material) -> Self {
        Self { s, u, v, point, material, normal: Vec3::ZERO, front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };
    }
}

pub trait Hit {
    /// Get a `HitRecord` for the ray and the object within the
    /// contour position interval `s_min` and `s_max`.
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord>;

    /// Determine a bounding `AABB` for the object between time `time0` and `time1`.
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
