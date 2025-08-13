use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    environment::Ray,
    objects::HitRecord,
    util::{Color, Vec3},
};

/// TODO: Add macros to autogenerate this stuff.
/// Especially for custom user materials
#[derive(Serialize, Deserialize, Debug)]
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

impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            Materials::Lambertian(l) => {
                Materials::Lambertian(Lambertian::new(l.albedo.clone(), l.scatter_prob))
            }
            Materials::Metal(m) => Materials::Metal(Metal::new(m.albedo.clone(), m.fuzz)),
            Materials::Dielectric(d) => Materials::Dielectric(Dielectric::new(d.refraction_index)),
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let mut scatter_dir = rec.normal() + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal().clone();
        }

        let scattered = Ray::new_at_time(rec.position(), scatter_dir, r_in.time());

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
/// normal. Fuzz allows the metal to not perfectly reflect
#[derive(Serialize, Deserialize, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Creates a new metal material.
    ///
    /// # Panics
    /// Panics if the fuzz factor is greater than 1 or less than 0.
    pub fn new(c: Color, fuzz: f64) -> Metal {
        assert!(fuzz <= 1.0, "A metal cannot have a fuzz factor above 1.0");
        assert!(fuzz >= 0.0, "A metal cannot have a fuzz factor below 0.0");
        Metal { albedo: c, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal());
        let reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        let scattered = Ray::new_at_time(rec.position(), reflected, r_in.time());
        *attenuation = self.albedo.clone();

        if scattered.direction().dot(&rec.normal()) > 0.0 {
            // Ensure that the ray is not going into the surface
            Some(scattered)
        } else {
            None // If it is just absorb the ray
        }
    }
}

/// A material representing water, or glass
#[derive(Serialize, Deserialize, Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    /// Creates a new dielectric with an index of
    /// refraction
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    /// Schlick's Approximation for the Fresnel factor
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    // figure out a way to get the refraction to realize what it is before it enters
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction().clone());
        let cos_theta = -unit_direction.clone().dot(&rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let mut rng = rand::rng();

        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rng.random::<f64>() {
                Vec3::reflect(&unit_direction, &rec.normal())
            } else {
                Vec3::refract(&unit_direction, &rec.normal(), ri)
            };

        Some(Ray::new_at_time(rec.position(), direction, r_in.time()))
    }
}
