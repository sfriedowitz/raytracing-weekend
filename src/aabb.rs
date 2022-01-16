use crate::{ray::Ray, vec::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    minimum: Vec3,
    maximum: Vec3,
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3::new(
            box0.minimum().x.min(box1.minimum().x),
            box0.minimum().y.min(box1.minimum().y),
            box0.minimum().y.min(box1.minimum().z),
        );
        let large = Vec3::new(
            box0.maximum().x.max(box1.maximum().x),
            box0.maximum().y.max(box1.maximum().y),
            box0.maximum().y.max(box1.maximum().z),
        );

        Self::new(small, large)
    }

    pub fn minimum(&self) -> Vec3 {
        self.minimum
    }

    pub fn maximum(&self) -> Vec3 {
        self.maximum
    }

    /// Determine if the ray hits the bounding box between contour positions `s_min` and `s_max`.
    pub fn hit(&self, r: &Ray, s_min: f64, s_max: f64) -> bool {
        let ray_origin = r.origin();
        let ray_direction = r.direction();

        for i in 0..3 {
            let inv_d = 1.0 / ray_direction[i];

            let s0 = (self.minimum[i] - ray_origin[i]) * inv_d;
            let s1 = (self.maximum[i] - ray_origin[i]) * inv_d;

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
