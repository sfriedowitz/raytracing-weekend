use glam::DVec3;

use crate::{color::Color, hit::HitRecord, ray::Ray, vec::VecOps};

/// Trait for scattering off a material.
pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

/// Enum defining all material textures on a hittable object.
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl From<Lambertian> for Material {
    fn from(l: Lambertian) -> Self {
        Self::Lambertian(l)
    }
}

impl From<Metal> for Material {
    fn from(m: Metal) -> Self {
        Self::Metal(m)
    }
}

impl From<Dielectric> for Material {
    fn from(d: Dielectric) -> Self {
        Self::Dielectric(d)
    }
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(l) => l.scatter(r_in, rec),
            Self::Metal(m) => m.scatter(r_in, rec),
            Self::Dielectric(d) => d.scatter(r_in, rec),
        }
    }
}

/// A material with diffuse scattering.
#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + DVec3::random_in_unit_sphere().normalize();
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction (equal to normal)
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

/// A material with reflective metal scattering.
#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalize();
        let scattered = Ray::new(rec.point, reflected + self.fuzz * DVec3::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

/// A material with mixed reflection and refraction.
#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ir: f64,
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        todo!()
    }
}
