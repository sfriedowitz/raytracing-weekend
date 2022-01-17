use std::ops::Range;

use rand::Rng;

use crate::{
    color::Color,
    ray::Ray,
    vec::{Vec3, VecOps},
};

#[derive(Clone, Debug)]
pub struct ViewOptions {
    pub background: Color,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub shutter_time: Range<f64>,
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub image_height: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
}

impl ViewOptions {
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

    pub fn with_shutter_time(mut self, time0: f64, time1: f64) -> Self {
        self.shutter_time = time0..time1;
        self
    }

    pub fn with_apsect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u64;
        self
    }

    pub fn with_image_width(mut self, image_width: u64) -> Self {
        self.image_width = image_width;
        self.image_height = ((image_width as f64) / self.aspect_ratio) as u64;
        self
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: u64) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_depth(mut self, max_depth: u64) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            background: Color::new(1.0, 1.0, 1.0),
            lookfrom: Vec3::new(13.0, 2.0, 3.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            aperture: 0.1,
            focus_dist: 10.0,
            shutter_time: (0.0..1.0),
            aspect_ratio: 16.0 / 9.0,
            image_width: 512,
            image_height: (512.0 * 9.0 / 16.0) as u64,
            samples_per_pixel: 200,
            max_depth: 50,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    shutter_time: Range<f64>,
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
        shutter_time: Range<f64>,
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
            shutter_time,
        }
    }

    pub fn from_options(opts: &ViewOptions) -> Self {
        Self::new(
            opts.lookfrom,
            opts.lookat,
            opts.vup,
            opts.vfov,
            opts.aperture,
            opts.focus_dist,
            opts.aspect_ratio,
            opts.shutter_time.clone(),
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = rand::thread_rng();

        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.shutter_time.clone()),
        )
    }
}
