use crate::{
    aabb::AABB,
    bvh::BVH,
    hit::{Hit, HitRecord},
    ray::Ray,
    rectangle::{XYRectangle, XZRectangle, YZRectangle},
    sphere::Sphere,
};

/// Enumeration of objects that can be hit by a ray.
#[derive(Clone, Debug)]
pub enum Hittable {
    Sphere(Sphere),
    XYRectangle(XYRectangle),
    XZRectangle(XZRectangle),
    YZRectangle(YZRectangle),
    BVH(BVH),
}

impl From<Sphere> for Hittable {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}

impl From<XYRectangle> for Hittable {
    fn from(r: XYRectangle) -> Self {
        Self::XYRectangle(r)
    }
}

impl From<XZRectangle> for Hittable {
    fn from(r: XZRectangle) -> Self {
        Self::XZRectangle(r)
    }
}

impl From<YZRectangle> for Hittable {
    fn from(r: YZRectangle) -> Self {
        Self::YZRectangle(r)
    }
}

impl From<BVH> for Hittable {
    fn from(b: BVH) -> Self {
        Self::BVH(b)
    }
}

impl Hit for Hittable {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(internal) => internal.hit(r, s_min, s_max),
            Self::XYRectangle(internal) => internal.hit(r, s_min, s_max),
            Self::XZRectangle(internal) => internal.hit(r, s_min, s_max),
            Self::YZRectangle(internal) => internal.hit(r, s_min, s_max),
            Self::BVH(internal) => internal.hit(r, s_min, s_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Self::Sphere(internal) => internal.bounding_box(time0, time1),
            Self::XYRectangle(internal) => internal.bounding_box(time0, time1),
            Self::XZRectangle(internal) => internal.bounding_box(time0, time1),
            Self::YZRectangle(internal) => internal.bounding_box(time0, time1),
            Self::BVH(internal) => internal.bounding_box(time0, time1),
        }
    }
}

/// Container for a collection of hittable objects.
pub type HittableList = Vec<Hittable>;

impl Hit for HittableList {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest = s_max;

        for hittable in self {
            if let Some(rec) = hittable.hit(r, s_min, closest) {
                closest = rec.s;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self.first() {
            // We have at least one element to create a box for
            Some(hittable) => match hittable.bounding_box(time0, time1) {
                // Accumulate the box by combining for each object
                // try_fold short-circuits and returns on None
                Some(init_bbox) => self.iter().skip(1).try_fold(init_bbox, |acc, hitable| {
                    hitable.bounding_box(time0, time1).map(|bbox| AABB::surrounding_box(acc, bbox))
                }),
                _ => None,
            },
            // Empty list, no box
            _ => None,
        }
    }
}
