use std::f64::consts::PI;

use rand::Rng;

use crate::{
    camera::{Camera, SamplingMethod, sample_square},
    objects::Hittables,
    scene::Skybox,
    utils::{Color, Interval, Point3, Vec3},
};

/// Ray represents a ray of light with a direction
/// and a starting point. Currently this takes ownership
/// of the origin and direction which may be a mistake
#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    /// Make a new ray at time 0.0
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            tm: 0.0,
        }
    }

    /// Make a new ray at a time
    pub fn new_at_time(origin: Point3, direction: Vec3, tm: f64) -> Ray {
        Ray {
            origin,
            direction,
            tm,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        let dir_clone: Vec3 = self.direction.clone();
        let orig_clone = self.origin.clone();

        let mult_dir = t * dir_clone;
        orig_clone + mult_dir
    }
}

/// Here are all the implementations pertaining to casting a ray for the camera
impl Camera {
    pub(super) fn cast_ray(
        &self,
        render_i: u32,
        render_j: u32,
        max_depth: u32,
        sb: &Skybox,
        world: &mut Hittables,
    ) -> Color {
        // Store the colors from each sample
        let mut sample_colors = Vec::new();
        let mut rng = rand::rng();

        // Compute current frame time:
        let current_time = (self.frame as f64) * (1.0 / self.frame_rate);
        // Compute the shutter length from the shutter angle
        let shutter_length = (self.shutter_angle / 360.0) * (1.0 / self.frame_rate);

        // loop and sample
        for _ in 0..self.samples {
            // Generate random time sample:
            let time_sample = current_time + rng.random_range(0.0..=shutter_length);

            // Get camera center at the time_sample
            let cc = self.get_from(time_sample);

            // Sample based on the method
            let offset = match self.sampling_method {
                SamplingMethod::Square => sample_square(),
            };

            let ps = self.get_pixel_pos(render_i, render_j, offset, time_sample);

            let ray_orig = if self.defocus_angle.get_angle() <= 0.0 {
                cc.clone()
            } else {
                self.defocus_disk_sample(time_sample)
            };

            let ray_dir = ps - ray_orig.clone();
            let ray_cast = Ray::new_at_time(ray_orig, ray_dir, time_sample);
            sample_colors.push(ray_color(ray_cast, max_depth, sb, world));
        }

        average_samples(sample_colors)
    }
}

// Function that causes ray bounces and computes the color of a ray_cast
pub fn ray_color(r: Ray, depth: u32, sb: &Skybox, world: &mut Hittables) -> Color {
    // If we have reached the max bounces we no longer
    // gather color contribution
    if depth == 0 {
        return Color::black();
    }
    // The interval starts at 0.001 to fix the 'shadow acne' behavior
    let hit = world.hit(&r, &Interval::new(0.001, f64::INFINITY));

    if let Some(h) = hit {
        let mut attenuation = Color::black();

        let scatter = h.material().scatter(&r, &h, &mut attenuation);

        if let Some(s) = scatter {
            return attenuation * ray_color(s, depth - 1, sb, world);
        }

        return Color::black();
    }

    match sb {
        Skybox::Spherical(sky) => {
            let unit_direction = r.direction().clone().unit_vector();
            let theta = unit_direction.x().atan2(unit_direction.z());
            let phi = unit_direction.y().asin();

            let u = (theta / (2.0 * PI)) + 0.5;
            let v = (phi / PI) + 0.5;

            // Clamp then scale with the skyboxes size:
            sky.get_color(u, v)
        }
        Skybox::Default => {
            let unit_direction = r.direction().clone().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}

pub(super) fn average_samples(sample_colors: Vec<Color>) -> Color {
    let mut r_tot = 0.0;
    let mut g_tot = 0.0;
    let mut b_tot = 0.0;

    let sample_count = sample_colors.len();

    for col in sample_colors {
        r_tot += col.r();
        g_tot += col.g();
        b_tot += col.b();
    }

    // Take the average
    r_tot /= sample_count as f64;
    g_tot /= sample_count as f64;
    b_tot /= sample_count as f64;

    Color::new(r_tot, g_tot, b_tot)
}
