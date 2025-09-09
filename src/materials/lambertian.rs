use std::sync::Arc;

use rand::Rng;

use crate::{
    camera::Ray,
    materials::Material,
    objects::HitRecord,
    textures::{solid_color::SolidColor, Textures},
    utils::{Color, Vec3},
};

/// Lambertian is a material that allows solid color
/// or texture wrapping for objects.
#[derive(Debug, Clone)]
pub struct Lambertian {
    tex: Arc<Textures>,
    scatter_prob: f64,
}

/// A perfect matte material. Prob gives the chance to
/// scatter a ray
impl Lambertian {
    pub fn new_from_color(c: Color, prob: f64) -> Lambertian {
        Lambertian {
            tex: Arc::new(Textures::SolidColor(SolidColor::new_from_color(c))),
            scatter_prob: prob,
        }
    }

    pub fn new_from_texture(tex: Arc<Textures>, prob: f64) -> Lambertian {
        Lambertian {
            tex,
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

        *attenuation = self
            .tex
            .value(rec.u_texture, rec.v_texture, &rec.position())
            / self.scatter_prob;

        let mut rng = rand::rng();

        if rng.random::<f64>() <= self.scatter_prob {
            Some(scattered)
        } else {
            None
        }
    }
}
