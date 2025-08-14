use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::util::{Color, Point3};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Textures {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl Textures {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Textures::SolidColor(s) => s.value(u, v, p),
            Textures::CheckerTexture(c) => c.value(u, v, p),
        }
    }
}

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// Creates a solid color texture from a color
    pub fn new_from_color(albedo: Color) -> SolidColor {
        SolidColor { albedo }
    }

    /// Creates a solid color texture from a rgb triple
    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Textures>,
    odd: Arc<Textures>,
}

impl CheckerTexture {
    pub fn new_from_textures(
        scale: f64,
        even: Arc<Textures>,
        odd: Arc<Textures>,
    ) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_color(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(Textures::SolidColor(SolidColor::new_from_color(c1))),
            odd: Arc::new(Textures::SolidColor(SolidColor::new_from_color(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
