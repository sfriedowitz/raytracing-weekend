use glam::DVec3;

use crate::{
    hit::{Hit, HitRecord},
    ray::Ray,
    sphere::Sphere,
};

/// Enum defining all objects that can be hit by a ray.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Hittable {
    Sphere(Sphere),
}

impl Hittable {
    pub fn sphere(center: DVec3, radius: f64) -> Self {
        Self::Sphere(Sphere::new(center, radius))
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
