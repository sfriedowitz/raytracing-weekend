use std::cmp::Ordering;

use rand::Rng;

use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    hittable::{Hittable, HittableList},
    ray::Ray,
};

#[derive(Clone, Debug)]
pub enum BVH {
    Leaf { object: Box<Hittable>, bbox: AABB },
    Internal { left: Box<Hittable>, right: Box<Hittable>, bbox: AABB },
}

impl BVH {
    pub fn new(mut objects: HittableList, time0: f64, time1: f64) -> Self {
        let mut rng = rand::thread_rng();
        let axis_cmp: usize = rng.gen_range(0..2);

        let comparator =
            |a: &Hittable, b: &Hittable| Self::box_compare(a, b, time0, time1, axis_cmp);
        objects.sort_unstable_by(comparator);

        let n = objects.len();
        match n {
            0 => panic!("Cannot construct a BVH from an empty object list."),
            1 => {
                let object = objects.pop().unwrap();
                let bbox = object.bounding_box(time0, time1).unwrap();
                Self::Leaf { object: Box::new(object), bbox }
            }
            _ => {
                let right = Self::new(objects.drain(n / 2..).collect(), time0, time1);
                let left = Self::new(objects, time0, time1);
                let bbox = AABB::surrounding_box(left.get_box(), right.get_box());

                Self::Internal { left: Box::new(left.into()), right: Box::new(right.into()), bbox }
            }
        }
    }

    fn box_compare(a: &Hittable, b: &Hittable, time0: f64, time1: f64, axis: usize) -> Ordering {
        let a_box = a.bounding_box(time0, time1);
        let b_box = b.bounding_box(time0, time1);

        if let (Some(ab), Some(bb)) = (a_box, b_box) {
            return ab.minimum()[axis].partial_cmp(&bb.minimum()[axis]).unwrap();
        };

        panic!("Cannot construct bounding box for objects in BVH node constructor.")
    }

    fn get_box(&self) -> AABB {
        match self {
            Self::Leaf { bbox, .. } => *bbox,
            Self::Internal { bbox, .. } => *bbox,
        }
    }
}

impl Hit for BVH {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        match self {
            Self::Leaf { object, .. } => object.hit(r, s_min, s_max),

            Self::Internal { left, right, bbox } => {
                // If the ray does not intersect AABB, return early
                if !bbox.hit(r, s_min, s_max) {
                    return None;
                }

                // Determine hit record between left and right
                // Why shift s_max from left hit???
                let left_rec = left.hit(r, s_min, s_max);
                let left_smax = left_rec.as_ref().map(|rec| rec.s).unwrap_or(s_max);
                let right_rec = right.hit(r, s_min, left_smax);

                if right_rec.is_some() {
                    right_rec
                } else {
                    left_rec
                }
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.get_box())
    }
}
