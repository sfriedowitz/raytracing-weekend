use rand::Rng;

use crate::{
    color::Color,
    ray::Ray,
    vec::{Vec3, VecOps},
};

pub struct CameraOptions {
    pub background: Color,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            background: Color::new(1.0, 1.0, 1.0),
            lookfrom: Vec3::new(13.0, 2.0, 3.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            aperture: 0.1,
            focus_dist: 10.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
        aspect_ratio: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
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

        Self {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            lens_radius: aperture / 2.0,
            cu,
            cv,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.time0..self.time1),
        )
    }
}
