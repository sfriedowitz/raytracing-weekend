use std::ops::Range;

use glam::DVec3;
use rand::{thread_rng, Rng};

/// Helper functions for generating 3D vectors.
pub trait VecOps {
    fn near_zero(self) -> bool;

    fn reflect(self, n: Self) -> Self;

    fn random(range: Range<f64>) -> Self;

    fn random_in_unit_sphere() -> Self;

    fn random_in_hemisphere(normal: Self) -> Self;
}

impl VecOps for DVec3 {
    fn random(range: Range<f64>) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());
        let z = rng.gen_range(range.clone());
        DVec3::new(x, y, z)
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let v = DVec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as normal
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    fn near_zero(self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }
}
