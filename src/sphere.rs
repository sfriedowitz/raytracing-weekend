use glam::DVec3;

use crate::{
    hit::{Hit, HitRecord},
    ray::Ray,
};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<crate::hit::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut t_root = (-half_b - sqrtd) / a;
        if t_root < t_min || t_root > t_max {
            t_root = (-half_b + sqrtd) / a;
            if t_root < t_min || t_root > t_max {
                return None;
            }
        }

        let root_point = r.at(t_root);
        let outward_normal = (root_point - self.center) / self.radius;

        let mut record = HitRecord::new(t_root, root_point);
        record.set_face_normal(r, outward_normal);

        Some(record)
    }
}
