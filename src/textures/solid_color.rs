use crate::{textures::Texture, utils::{Color, Point3}};

/// A texture representing a solid color for
/// an object
#[derive(Debug, Clone)]
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
