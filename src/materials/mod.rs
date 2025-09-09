use crate::{
    camera::Ray,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    objects::HitRecord,
    utils::Color,
};

pub mod dielectric;
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
}

impl Materials {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec, attenuation),
            Materials::Metal(m) => m.scatter(r_in, rec, attenuation),
            Materials::Dielectric(d) => d.scatter(r_in, rec, attenuation),
        }
    }
}

/// This trait defines the ray scattering
/// behavior of a material. Scatter returns an option
/// representing if the ray scattered or was absorbed (None)
/// and updates a HitRecord describing the hit
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}
