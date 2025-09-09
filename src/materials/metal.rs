use crate::{
    camera::Ray,
    materials::Material,
    objects::HitRecord,
    utils::{Color, Vec3},
};

/// A reflective material, bounces rays against the
/// normal. Fuzz allows the metal to not perfectly reflect
#[derive(Debug, Clone)]
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
