use glam::DVec3;

use crate::{
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
};

/// Container for a collection of hittable objects.
pub type World = Vec<Hittable>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest = t_max;

        for hittable in self {
            if let Some(rec) = hittable.hit(r, t_min, closest) {
                closest = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}

/// Enum defining all objects that can be hit by a ray.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Hittable {
    Sphere(Sphere),
}

impl From<Sphere> for Hittable {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}

impl Hit for Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(r, t_min, t_max),
            _ => None,
        }
    }
}

/// A hittable sphere with a radius, center, and material texture.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    radius: f64,
    center: DVec3,
    material: Material,
}

impl Sphere {
    pub fn new(radius: f64, center: DVec3, material: Material) -> Self {
        Self { radius, center, material }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

        let mut rec = HitRecord::new(t_root, root_point, self.material);
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
