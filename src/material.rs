use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    environment::Ray,
    objects::HitRecord,
    util::{Color, Vec3},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Materials {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec, attenuation),
            Materials::Metal(m) => m.scatter(r_in, rec, attenuation),
        }
    }
}

impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            Materials::Lambertian(l) => {
                Materials::Lambertian(Lambertian::new(l.albedo.clone(), l.scatter_prob))
            }
            Materials::Metal(m) => Materials::Metal(Metal::new(m.albedo.clone())),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Lambertian {
    albedo: Color,
    scatter_prob: f64,
}

/// A perfect matte material. Prob gives the chance to
/// scatter a ray
impl Lambertian {
    pub fn new(c: Color, prob: f64) -> Lambertian {
        Lambertian {
            albedo: c,
            scatter_prob: prob,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let mut scatter_dir = rec.normal() + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal().clone();
        }

        let scattered = Ray::new(rec.position(), scatter_dir);

        *attenuation = self.albedo.clone() / self.scatter_prob;

        let mut rng = rand::rng();

        if rng.random::<f64>() <= self.scatter_prob {
            Some(scattered)
        } else {
            None
        }
    }
}

/// A reflective material, bounces rays against the
/// normal.
#[derive(Serialize, Deserialize, Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(c: Color) -> Metal {
        Metal { albedo: c }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected = Vec3::reflect_vec3(r_in.direction(), &rec.normal());

        let scattered = Ray::new(rec.position(), reflected);
        *attenuation = self.albedo.clone();

        Some(scattered) // Reflective does not absorb light
    }
}
