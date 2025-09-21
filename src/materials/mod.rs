use crate::{
    camera::Ray,
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    objects::HitRecord,
    utils::{Color, Point3},
};

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

/// A wrapper for materials in the renderer, this handles dispatching
/// calls to individual materials. It also allows for precise control
/// over what material something is. TODO: Possible make derive macros for these?
#[derive(Debug, Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Materials {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec, attenuation),
            Materials::Metal(m) => m.scatter(r_in, rec, attenuation),
            Materials::Dielectric(d) => d.scatter(r_in, rec, attenuation),
            Materials::DiffuseLight(dl) => dl.scatter(r_in, rec, attenuation),
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Materials::Lambertian(l) => l.emitted(u, v, p),
            Materials::Metal(m) => m.emitted(u, v, p),
            Materials::Dielectric(d) => d.emitted(u, v, p),
            Materials::DiffuseLight(dl) => dl.emitted(u, v, p),
        }
    }
}

/// This trait defines the ray scattering
/// behavior of a material. Scatter returns an option
/// representing if the ray scattered or was absorbed (None)
/// and updates a HitRecord describing the hit
pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color) -> Option<Ray> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::black()
    }
}
