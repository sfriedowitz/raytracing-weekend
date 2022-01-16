use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
    vec::Vec3,
};

#[derive(Clone, Debug)]
pub struct XYRectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

impl XYRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        Self { x0, x1, y0, y1, k, material }
    }
}

impl Hit for XYRectangle {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let s = (self.k - r.origin().z) / r.direction().z;
        if s < s_min || s > s_max {
            return None;
        }

        let x = r.origin().x + s * r.direction().x;
        let y = r.origin().y + s * r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let point = r.at(s);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

        let mut rec = HitRecord::new(s, u, v, point, self.material.clone());
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have a non-zero width in each dimension,
        // so pad the bounding box a little bit
        let min = Vec3::new(self.x0, self.y0, self.k - 1e-3 * self.k);
        let max = Vec3::new(self.x1, self.y1, self.k + 1e-3 * self.k);
        Some(AABB::new(min, max))
    }
}

#[derive(Clone, Debug)]
pub struct XZRectangle {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl XZRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self { x0, x1, z0, z1, k, material }
    }
}

impl Hit for XZRectangle {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let s = (self.k - r.origin().y) / r.direction().y;
        if s < s_min || s > s_max {
            return None;
        }

        let x = r.origin().x + s * r.direction().x;
        let z = r.origin().z + s * r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = r.at(s);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

        let mut rec = HitRecord::new(s, u, v, point, self.material.clone());
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have a non-zero width in each dimension,
        // so pad the bounding box a little bit
        let min = Vec3::new(self.x0, self.k - 1e-3 * self.k, self.z0);
        let max = Vec3::new(self.x1, self.k + 1e-3 * self.k, self.z1);
        Some(AABB::new(min, max))
    }
}

#[derive(Clone, Debug)]
pub struct YZRectangle {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl YZRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self { y0, y1, z0, z1, k, material }
    }
}

impl Hit for YZRectangle {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let s = (self.k - r.origin().x) / r.direction().x;
        if s < s_min || s > s_max {
            return None;
        }

        let y = r.origin().y + s * r.direction().y;
        let z = r.origin().z + s * r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let point = r.at(s);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

        let mut rec = HitRecord::new(s, u, v, point, self.material.clone());
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have a non-zero width in each dimension,
        // so pad the bounding box a little bit
        let min = Vec3::new(self.k - 1e-3 * self.k, self.y0, self.z0);
        let max = Vec3::new(self.k + 1e-3 * self.k, self.y1, self.z1);
        Some(AABB::new(min, max))
    }
}
