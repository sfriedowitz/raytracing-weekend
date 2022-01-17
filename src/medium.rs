use rand::Rng;

use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    hittable::Hittable,
    material::Material,
    ray::Ray,
};

#[derive(Clone, Debug)]
pub struct ConstantMedium {
    boundary: Box<Hittable>,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: impl Into<Hittable>, phase_function: impl Into<Material>, d: f64) -> Self {
        let neg_inv_density = -1.0 / d;
        Self {
            boundary: Box::new(boundary.into()),
            phase_function: phase_function.into(),
            neg_inv_density,
        }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY);
        let rec1_s = rec1.as_ref().map(|rec| rec.s).unwrap_or(f64::NEG_INFINITY);
        let rec2 = self.boundary.hit(r, rec1_s + 0.0001, f64::INFINITY);

        if rec1.is_none() || rec2.is_none() {
            return None;
        }

        let mut rec1 = rec1.unwrap();
        let mut rec2 = rec2.unwrap();

        if rec1.s < s_min {
            rec1.s = s_min;
        }

        if rec2.s > s_max {
            rec2.s = s_max;
        }

        if rec1.s >= rec2.s {
            return None;
        }

        if rec1.s < 0.0 {
            rec1.s = 0.0;
        }

        let mut rng = rand::thread_rng();
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.s - rec1.s) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let rec_s = rec1.s + hit_distance / ray_length;
        let rec_point = r.at(rec_s);
        let rec = HitRecord::new(rec_s, 0.0, 0.0, rec_point, self.phase_function.clone());
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
