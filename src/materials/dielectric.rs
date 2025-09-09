use rand::Rng;

use crate::{camera::Ray, materials::Material, objects::HitRecord, utils::{Color, Vec3}};

/// A material representing water, or glass
/// 
/// TODO: Beer's law and volumetric meshes
#[derive(Debug, Clone)]
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
