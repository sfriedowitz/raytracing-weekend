use glam::DVec3;

use crate::{ray::Ray, vec::VecOps};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: DVec3,
    lower_left_corner: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    cu: DVec3,
    cv: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: DVec3,
        lookat: DVec3,
        vup: DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalize();
        let cu = vup.cross(cw).normalize();
        let cv = cw.cross(cu);
        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu: cu,
            cv: cv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * DVec3::random_in_unit_disk();
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
