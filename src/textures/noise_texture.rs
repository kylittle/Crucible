use crate::{
    textures::Texture,
    utils::{Color, Perlin},
};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &crate::utils::Point3) -> crate::utils::Color {
        return Color::white() * self.noise.noise(p);
    }
}
