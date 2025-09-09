use crate::{
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, solid_color::SolidColor,
    },
    utils::{Color, Point3},
};

pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;

#[derive(Debug, Clone)]
pub enum Textures {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
}

impl Textures {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Textures::SolidColor(s) => s.value(u, v, p),
            Textures::CheckerTexture(c) => c.value(u, v, p),
            Textures::ImageTexture(i) => i.value(u, v, p),
        }
    }
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
