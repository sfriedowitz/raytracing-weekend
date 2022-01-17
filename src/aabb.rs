use crate::{ray::Ray, vec::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3::new(
            box0.min().x.min(box1.min().x),
            box0.min().y.min(box1.min().y),
            box0.min().y.min(box1.min().z),
        );
        let large = Vec3::new(
            box0.max().x.max(box1.max().x),
            box0.max().y.max(box1.max().y),
            box0.max().y.max(box1.max().z),
        );

        Self::new(small, large)
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    /// Determine if the ray hits the bounding box between contour positions `s_min` and `s_max`.
    pub fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> bool {
        let ray_origin = r.origin();
        let ray_direction = r.direction();

        for i in 0..3 {
            let inv_d = 1.0 / ray_direction[i];

            let s0 = (self.min[i] - ray_origin[i]) * inv_d;
            let s1 = (self.max[i] - ray_origin[i]) * inv_d;

            let (s0, s1) = if inv_d < 0.0 { (s1, s0) } else { (s0, s1) };

            let s_min = s_min.max(s0);
            let s_max = s_max.min(s1);

            if s_max <= s_min {
                return false;
            }
        }

        true
    }
}
