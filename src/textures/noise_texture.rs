use crate::{
    textures::Texture,
    utils::{Color, Perlin, Point3},
};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        // TODO: Figure out how this should work
        // Basic noise example
        // Color::white() * 0.5 * (1.0 + self.noise.noise(&(self.scale * p.clone())))
        // Turbulence example
        // Color::white() * self.noise.turbulence(p, 7)
        // Marbling example
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turbulence(p, 7)).sin())
    }
}
