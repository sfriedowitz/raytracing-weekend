use rand::Rng;

use crate::{
    color::Color,
    ray::Ray,
    vec::{Vec3, VecOps},
};

#[derive(Clone, Copy, Debug)]
pub struct CameraOptions {
    pub background: Color,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub aspect_ratio: f64,
    pub time0: f64,
    pub time1: f64,
}

impl CameraOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn with_lookfrom(mut self, lookfrom: Vec3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(mut self, lookat: Vec3) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn with_vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn with_vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn with_aperture(mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn with_apsect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_time0(mut self, time0: f64) -> Self {
        self.time0 = time0;
        self
    }

    pub fn with_time1(mut self, time1: f64) -> Self {
        self.time1 = time1;
        self
    }
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
            aspect_ratio: 16.0 / 9.0,
            time0: 0.0,
            time1: 1.0,
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

    pub fn from_options(opts: CameraOptions) -> Self {
        Self::new(
            opts.lookfrom,
            opts.lookat,
            opts.vup,
            opts.vfov,
            opts.aperture,
            opts.focus_dist,
            opts.aspect_ratio,
            opts.time0,
            opts.time1,
        )
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
