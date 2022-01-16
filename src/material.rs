use rand::{thread_rng, Rng};

use crate::{
    color::Color,
    hit::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture, TextureColor},
    vec::Vec3,
    vec::VecOps,
};

/// Trait for scattering off a material.
pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, point: Vec3, u: f64, v: f64) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

/// Enumeration of material textures on a hittable object.
#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl From<Lambertian> for Material {
    fn from(material: Lambertian) -> Self {
        Self::Lambertian(material)
    }
}

impl From<Metal> for Material {
    fn from(material: Metal) -> Self {
        Self::Metal(material)
    }
}

impl From<Dielectric> for Material {
    fn from(material: Dielectric) -> Self {
        Self::Dielectric(material)
    }
}

impl From<DiffuseLight> for Material {
    fn from(material: DiffuseLight) -> Self {
        Self::DiffuseLight(material)
    }
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(inner) => inner.scatter(r_in, rec),
            Self::Metal(inner) => inner.scatter(r_in, rec),
            Self::Dielectric(inner) => inner.scatter(r_in, rec),
            Self::DiffuseLight(inner) => inner.scatter(r_in, rec),
        }
    }

    fn emitted(&self, point: Vec3, u: f64, v: f64) -> Color {
        match self {
            Self::Lambertian(inner) => inner.emitted(point, u, v),
            Self::Metal(inner) => inner.emitted(point, u, v),
            Self::Dielectric(inner) => inner.emitted(point, u, v),
            Self::DiffuseLight(inner) => inner.emitted(point, u, v),
        }
    }
}

/// A material with diffuse scattering.
#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Self {
        Self { albedo: texture }
    }

    pub fn from_color(color: Color) -> Self {
        Self::new(SolidColor::new(color).into())
    }
}

impl From<Color> for Lambertian {
    fn from(color: Color) -> Self {
        Self::new(SolidColor::new(color).into())
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalize();
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction (equal to normal)
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_direction, r_in.time());
        let attenuation = self.albedo.color_value(rec.point, rec.u, rec.v);

        Some((attenuation, scattered))
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
        let scattered =
            Ray::new(rec.point, reflected + self.fuzz * Vec3::random_in_unit_sphere(), r_in.time());

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

        let scattered = Ray::new(rec.point, direction, r_in.time());

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn new(emit: Texture) -> Self {
        Self { emit }
    }
}

impl From<Color> for DiffuseLight {
    fn from(color: Color) -> Self {
        Self::new(SolidColor::new(color).into())
    }
}

impl Scatter for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, point: Vec3, u: f64, v: f64) -> Color {
        self.emit.color_value(point, u, v)
    }
}
