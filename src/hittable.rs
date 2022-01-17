use crate::{
    aabb::AABB,
    bvh::BVH,
    cuboid::Cuboid,
    hit::{Hit, HitRecord},
    ray::Ray,
    rectangle::{XYRectangle, XZRectangle, YZRectangle},
    rotate::RotateY,
    sphere::Sphere,
    translate::Translate,
};

/// Enumeration of objects that can be hit by a ray.
#[derive(Clone, Debug)]
pub enum Hittable {
    Sphere(Sphere),
    XYRectangle(XYRectangle),
    XZRectangle(XZRectangle),
    YZRectangle(YZRectangle),
    Cuboid(Cuboid),
    Translate(Translate),
    RotateY(RotateY),
    BVH(BVH),
}

impl From<Sphere> for Hittable {
    fn from(inner: Sphere) -> Self {
        Self::Sphere(inner)
    }
}

impl From<XYRectangle> for Hittable {
    fn from(inner: XYRectangle) -> Self {
        Self::XYRectangle(inner)
    }
}

impl From<XZRectangle> for Hittable {
    fn from(inner: XZRectangle) -> Self {
        Self::XZRectangle(inner)
    }
}

impl From<YZRectangle> for Hittable {
    fn from(inner: YZRectangle) -> Self {
        Self::YZRectangle(inner)
    }
}

impl From<Cuboid> for Hittable {
    fn from(inner: Cuboid) -> Self {
        Self::Cuboid(inner)
    }
}

impl From<Translate> for Hittable {
    fn from(inner: Translate) -> Self {
        Self::Translate(inner)
    }
}

impl From<RotateY> for Hittable {
    fn from(inner: RotateY) -> Self {
        Self::RotateY(inner)
    }
}

impl From<BVH> for Hittable {
    fn from(inner: BVH) -> Self {
        Self::BVH(inner)
    }
}

impl Hit for Hittable {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(inner) => inner.hit(r, s_min, s_max),
            Self::XYRectangle(inner) => inner.hit(r, s_min, s_max),
            Self::XZRectangle(inner) => inner.hit(r, s_min, s_max),
            Self::YZRectangle(inner) => inner.hit(r, s_min, s_max),
            Self::Cuboid(inner) => inner.hit(r, s_min, s_max),
            Self::Translate(inner) => inner.hit(r, s_min, s_max),
            Self::RotateY(inner) => inner.hit(r, s_min, s_max),
            Self::BVH(inner) => inner.hit(r, s_min, s_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Self::Sphere(inner) => inner.bounding_box(time0, time1),
            Self::XYRectangle(inner) => inner.bounding_box(time0, time1),
            Self::XZRectangle(inner) => inner.bounding_box(time0, time1),
            Self::YZRectangle(inner) => inner.bounding_box(time0, time1),
            Self::Cuboid(inner) => inner.bounding_box(time0, time1),
            Self::Translate(inner) => inner.bounding_box(time0, time1),
            Self::RotateY(inner) => inner.bounding_box(time0, time1),
            Self::BVH(inner) => inner.bounding_box(time0, time1),
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
