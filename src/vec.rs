use std::ops::Range;

use glam::DVec3;
use rand::{thread_rng, Rng};

/// Helper functions for generating 3D vectors.
pub trait VecUtils {
    fn random(range: Range<f64>) -> Self;

    fn random_in_unit_sphere() -> Self;
}

impl VecUtils for DVec3 {
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
}
