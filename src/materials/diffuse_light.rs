use std::sync::Arc;

use crate::{
    materials::Material,
    textures::{Textures, solid_color::SolidColor},
    utils::{Color, Point3},
};

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    tex: Arc<Textures>,
}

impl DiffuseLight {
    pub fn new_from_texture(tex: Arc<Textures>) -> DiffuseLight {
        DiffuseLight { tex }
    }

    pub fn new_from_color(col: Color) -> DiffuseLight {
        DiffuseLight {
            tex: Arc::new(Textures::SolidColor(SolidColor::new_from_color(col))),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
