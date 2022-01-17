use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    hittable::Hittable,
    ray::Ray,
    vec::Vec3,
};

#[derive(Clone, Debug)]
pub struct RotateY {
    object: Box<Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: impl Into<Hittable>, angle: f64) -> Self {
        let object = Box::new(object.into());

        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object
            .bounding_box(0.0, 1.0)
            .expect("Cannot construct bounding box for object to be rotated.");

        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.max().x + ((1 - i) as f64) * bbox.min().x;
                    let y = (j as f64) * bbox.max().y + ((1 - j) as f64) * bbox.min().y;
                    let z = (k as f64) * bbox.max().z + ((1 - k) as f64) * bbox.min().z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let test_vec = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = min[c].min(test_vec[c]);
                        max[c] = max[c].max(test_vec[c]);
                    }
                }
            }
        }

        let min_vec = Vec3::from_slice(&min);
        let max_vec = Vec3::from_slice(&max);
        let rotated_bbox = AABB::new(min_vec, max_vec);

        Self { object, sin_theta, cos_theta, bbox: rotated_bbox }
    }
}

impl Hit for RotateY {
    fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> Option<HitRecord> {
        let origin = r.origin();
        let direction = r.direction();

        let rotated_ox = self.cos_theta * origin.x - self.sin_theta * origin.z;
        let rotated_oz = self.sin_theta * origin.x + self.cos_theta * origin.z;

        let rotated_dx = self.cos_theta * direction.x - self.sin_theta * direction.z;
        let rotated_dz = self.sin_theta * direction.x + self.cos_theta * direction.z;

        let rotated_origin = Vec3::new(rotated_ox, origin.y, rotated_oz);
        let rotated_direction = Vec3::new(rotated_dx, direction.y, rotated_dz);
        let rotated_ray = Ray::new(rotated_origin, rotated_direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotated_ray, s_min, s_max) {
            let p = rec.point;
            let n = rec.normal;

            let rotated_px = self.cos_theta * p.x + self.sin_theta * p.z;
            let rotated_pz = -self.sin_theta * p.x + self.cos_theta * p.z;

            let rotated_nx = self.cos_theta * n.x + self.sin_theta * n.z;
            let rotated_nz = -self.sin_theta * n.x + self.cos_theta * n.z;

            let rotated_point = Vec3::new(rotated_px, p.y, rotated_pz);
            let rotated_normal = Vec3::new(rotated_nx, n.y, rotated_nz);

            rec.point = rotated_point;
            rec.set_face_normal(&rotated_ray, rotated_normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
