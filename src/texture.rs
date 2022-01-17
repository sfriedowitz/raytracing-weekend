use image::GenericImageView;

use crate::{color::Color, perlin::Perlin, vec::Vec3};

pub trait TextureColor {
    fn color_value(&self, point: Vec3, u: f64, v: f64) -> Color;
}

#[derive(Clone, Debug)]
pub enum Texture {
    Solid(SolidColor),
    Checker(CheckerTexture),
    Noise(NoiseTexture),
    Image(ImageTexture),
}

impl From<SolidColor> for Texture {
    fn from(texture: SolidColor) -> Self {
        Self::Solid(texture)
    }
}

impl From<CheckerTexture> for Texture {
    fn from(texture: CheckerTexture) -> Self {
        Self::Checker(texture)
    }
}

impl From<NoiseTexture> for Texture {
    fn from(texture: NoiseTexture) -> Self {
        Self::Noise(texture)
    }
}

impl From<ImageTexture> for Texture {
    fn from(texture: ImageTexture) -> Self {
        Self::Image(texture)
    }
}

impl TextureColor for Texture {
    fn color_value(&self, point: Vec3, u: f64, v: f64) -> Color {
        match self {
            Self::Solid(inner) => inner.color_value(point, u, v),
            Self::Checker(inner) => inner.color_value(point, u, v),
            Self::Noise(inner) => inner.color_value(point, u, v),
            Self::Image(inner) => inner.color_value(point, u, v),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl TextureColor for SolidColor {
    fn color_value(&self, _point: Vec3, _u: f64, _v: f64) -> Color {
        self.color
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(even: impl Into<Texture>, odd: impl Into<Texture>) -> Self {
        Self { even: Box::new(even.into()), odd: Box::new(odd.into()) }
    }

    pub fn from_colors(even: Color, odd: Color) -> Self {
        Self::new(SolidColor::new(even), SolidColor::new(odd))
    }
}

impl TextureColor for CheckerTexture {
    fn color_value(&self, point: Vec3, u: f64, v: f64) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.color_value(point, u, v)
        } else {
            self.even.color_value(point, u, v)
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl TextureColor for NoiseTexture {
    fn color_value(&self, point: Vec3, _u: f64, _v: f64) -> Color {
        let sin_arg = self.scale * point.z + 10.0 * self.noise.turb(point, 1);
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + sin_arg.sin())
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(filepath: &str) -> Self {
        match image::open(filepath) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let data = img.into_bytes();
                Self { data, width: width as usize, height: height as usize }
            }
            Err(_) => Self { data: Vec::new(), width: 0, height: 0 },
        }
    }
}

impl TextureColor for ImageTexture {
    fn color_value(&self, _point: Vec3, u: f64, v: f64) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let i = ((u * self.width as f64) as usize).min(self.width - 1);
        let j = (((1.0 - v) * self.height as f64) as usize).min(self.height - 1);
        let idx = 3 * i + 3 * self.width * j;

        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;

        Color::new(r, g, b)
    }
}
