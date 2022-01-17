use std::f64::consts::PI;

use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
    vec::Vec3,
};

/// A hittable sphere that moves from one point to another within the shutter frame.
///
/// The sphere can be treated as stationary by supplying identical center positions for the frame.
#[derive(Clone, Debug)]
pub struct Sphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
    ) -> Self {
        Self { center0, center1, time0, time1, radius, material }
    }

    pub fn stationary(center: Vec3, radius: f64, material: Material) -> Self {
        Self::new(center, center, 0.0, 0.0, radius, material)
    }

    pub fn center(&self, time: f64) -> Vec3 {
        let dtime = self.time1 - self.time0;
        let dcenter = self.center1 - self.center0;
        if dtime == 0.0 || dcenter == Vec3::ZERO {
            self.center0
        } else {
            self.center0 + ((time - self.time0) / dtime) * dcenter
        }
    }

    pub fn get_uv(point: Vec3) -> (f64, f64) {
        // Point: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut s_root = (-half_b - sqrtd) / a;
        if s_root < s_min || s_root > s_max {
            s_root = (-half_b + sqrtd) / a;
            if s_root < s_min || s_root > s_max {
                return None;
            }
        }

        let root_point = r.at(s_root);
        let outward_normal = (root_point - self.center(r.time())) / self.radius;
        let (u, v) = Self::get_uv(outward_normal);

        let mut rec = HitRecord::new(s_root, u, v, root_point, self.material.clone());
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let r = self.radius;
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(r, r, r),
            self.center(time0) + Vec3::new(r, r, r),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(r, r, r),
            self.center(time1) + Vec3::new(r, r, r),
        );
        Some(AABB::surrounding_box(box0, box1))
    }
}
