use glam::DVec3;
use rand::{thread_rng, Rng};

use crate::{color::Color, hit::HitRecord, ray::Ray, vec::VecOps};

/// Trait for scattering off a material.
pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

/// Enumeration of material textures on a hittable object.
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
            Self::Lambertian(inner) => inner.scatter(r_in, rec),
            Self::Metal(inner) => inner.scatter(r_in, rec),
            Self::Dielectric(inner) => inner.scatter(r_in, rec),
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

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self { ir: index_of_refraction }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = r_in.direction().normalize();

        // Solution sometimes cannot refract due to total internal reflection
        // Solve for sin(theta) to determine when to reflect/refract
        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.point, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
