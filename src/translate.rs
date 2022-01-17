use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    hittable::Hittable,
    ray::Ray,
    vec::Vec3,
};

#[derive(Clone, Debug)]
pub struct Translate {
    object: Box<Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: impl Into<Hittable>, offset: Vec3) -> Self {
        Self { object: Box::new(object.into()), offset }
    }
}

impl Hit for Translate {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&moved_ray, s_min, s_max) {
            rec.point += self.offset;
            rec.set_face_normal(&moved_ray, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.object
            .bounding_box(time0, time1)
            .map(|base_box| AABB::new(base_box.min() + self.offset, base_box.max() + self.offset))
    }
}
