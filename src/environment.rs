use std::f64::INFINITY;

use indicatif::ProgressBar;
use rand::Rng;

use crate::{
    objects::Hittable,
    util::{Color, Interval, Point3, Vec3},
};

/// Ray represents a ray of light with a direction
/// and a starting point. Currently this takes ownership
/// of the origin and direction which may be a mistake
#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Make a new ray
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        let dir_clone: Vec3 = self.direction.clone();
        let orig_clone = self.origin.clone();

        let mult_dir = t * dir_clone;
        orig_clone + mult_dir
    }
}

struct Viewport {
    viewport_height: f64,
    viewport_width: f64,
    image_height: u32,
    image_width: u32,
}

impl Viewport {
    /// Make a new Viewport in the environment, it will have an
    /// aspect ratio and a size in pixels.
    fn new(aspect_ratio: f64, image_width: u32) -> Viewport {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = image_height.clamp(1, u32::MAX);

        // Arbitrary value from book. Maybe try changing it?
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        Viewport {
            viewport_height,
            viewport_width,
            image_height,
            image_width,
        }
    }
}

pub struct Camera {
    viewport: Viewport,
    focal_length: f64,
    camera_center: Point3,
    samples: u32,
    sampling_method: SamplingMethod,
    max_depth: u32,
}

pub enum SamplingMethod {
    Square,
}

impl Camera {
    /// Pass in None for the parameter camera_center to
    /// get (0, 0, 0) or specify your own center
    /// *TODO*: make sure camera center actually works and add
    /// support for camera rotations and movement
    pub fn new(aspect_ratio: f64, image_width: u32) -> Camera {
        let v = Viewport::new(aspect_ratio, image_width);
        let cc = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let samples = 10;
        let sampling_method = SamplingMethod::Square;
        let max_depth = 10;

        Camera {
            viewport: v,
            focal_length,
            camera_center: cc,
            samples,
            sampling_method,
            max_depth,
        }
    }

    /// Sets the cameras center position in the image
    pub fn set_loc(&mut self, loc: Point3) {
        self.camera_center = loc;
    }

    /// Sets the cameras distance from the viewport
    pub fn set_focal_length(&mut self, fl: f64) {
        self.focal_length = fl;
    }

    /// Sets the number of samples. This option can be
    /// expensive so set to a high value with caution.
    ///
    /// #Panics:
    /// This panics if s is not a positive integer.
    pub fn set_samples(&mut self, s: u32) {
        assert!(
            s > 0,
            "The camera must have a positive number of samples. {s} is invalid."
        );

        self.samples = s;
    }

    /// Sets the number of how many recursive calls the renderer
    /// will make when a ray bounces off a surface
    pub fn set_max_depth(&mut self, md: u32) {
        self.max_depth = md;
    }

    // TODO: Inlined for efficiency might not work when
    // the camera can move
    /// Vector representing the horizontal viewport edge
    #[inline]
    fn viewport_u(&self) -> Vec3 {
        Vec3::new(self.viewport.viewport_width, 0.0, 0.0)
    }

    /// Vector representing the vertical viewport edge. It is
    /// negative since the coordinate for the image are opposite
    /// to the camera (we want our vec to point down)
    #[inline]
    fn viewport_v(&self) -> Vec3 {
        Vec3::new(0.0, -self.viewport.viewport_height, 0.0)
    }

    /// Subdivide the length of our viewport by pixels
    /// this gets the vector between two pixels in the
    /// x-axis.
    #[inline]
    fn pixel_delta_u(&self) -> Vec3 {
        self.viewport_u() / self.viewport.image_width as f64
    }

    /// Subdivide the length of our viewport by pixels
    /// this gets the vector between two pixels in the
    /// y-axis.
    #[inline]
    fn pixel_delta_v(&self) -> Vec3 {
        self.viewport_v() / self.viewport.image_height as f64
    }

    /// Compute the upper left hand corner. This uses the
    /// cameras position to move to the upper left. However
    /// the / 2.0 on the last two lines breaks generality of
    /// camera position. *TODO*: Fix this (maybe not? the viewport probably moves
    /// with the camera right?)
    #[inline]
    fn viewport_upperleft(&self) -> Point3 {
        let cc = self.camera_center.clone();
        cc - Point3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0
    }

    #[inline]
    fn pixel_start_location(&self) -> Point3 {
        self.viewport_upperleft() + 0.5 * (self.pixel_delta_u() + self.pixel_delta_v())
    }

    /// The camera can take an ij pair in the image and
    /// calculate its position relative to the camera
    fn get_pixel_pos(&self, i: u32, j: u32, offset: Point3) -> Point3 {
        self.pixel_start_location()
            + ((i as f64 + offset.x()) * self.pixel_delta_u())
            + ((j as f64 + offset.y()) * self.pixel_delta_v())
    }

    fn cast_ray<T: Hittable>(
        &self,
        render_i: u32,
        render_j: u32,
        max_depth: u32,
        world: &T,
    ) -> Color {
        let cc = self.camera_center.clone();

        // Store the colors from each sample
        let mut sample_colors = Vec::new();

        // loop and sample
        for _ in 0..self.samples {
            // Sample based on the method
            let offset = match self.sampling_method {
                SamplingMethod::Square => sample_square(),
            };

            let ps = self.get_pixel_pos(render_i, render_j, offset);

            let ray_dir = ps - cc.clone();
            let ray_cast = Ray::new(cc.clone(), ray_dir);

            sample_colors.push(self.ray_color(ray_cast, max_depth, world));
        }

        average_samples(sample_colors)
    }

    fn ray_color<T: Hittable>(&self, r: Ray, depth: u32, world: &T) -> Color {
        // If we have reached the max bounces we no longer
        // gather color contribution
        if depth == 0 {
            return Color::black();
        }

        // The interval starts at 0.001 to fix the 'shadow acne' behavior
        let hit = world.hit(&r, &Interval::new(0.001, INFINITY));

        if let Some(h) = hit {
            let mut attenuation = Color::black();

            let scatter = h.material().scatter(&r, &h, &mut attenuation);

            if let Some(s) = scatter {
                return attenuation * self.ray_color(s, depth - 1, world);
            }

            return Color::black();
        }

        let unit_direction = r.direction().clone().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }

    /// This causes the camera to render an image to stdout
    /// TODO: possibly change this so it renders to a file based on an input
    pub fn render<T: Hittable>(&self, world: &T) {
        let iw = self.viewport.image_width;
        let ih = self.viewport.image_height;

        // Render

        println!("P3\n{iw} {ih}\n255");

        let bar = ProgressBar::new((ih * iw).into());

        for j in 0..ih {
            for i in 0..iw {
                // decimal values for each color from 0.0 to 1.0
                let c = self.cast_ray(i, j, self.max_depth, world);

                println!("{c}"); // TODO: Output to a file

                bar.inc(1);
            }
        }

        bar.finish();
    }
}

// Helper functions
fn average_samples(sample_colors: Vec<Color>) -> Color {
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

/// Later change sampling so I can modify the sampling method
/// to test different effects on image quality
#[inline]
fn sample_square() -> Vec3 {
    // TODO: RNG may be too slow. But it is thread safe for the future
    let mut rng = rand::rng();
    let x = rng.random::<f64>() - 0.5;
    let y = rng.random::<f64>() - 0.5;

    Vec3::new(x, y, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_test() {
        let r = Ray::new(Point3::origin(), Point3::new(2.0, -3.0, 1.5));

        assert_eq!(r.at(2.0), Point3::new(4.0, -6.0, 3.0));
    }

    #[test]
    fn average_color_test() {
        let cv = vec![Color::new(0.0, 1.0, 0.0), Color::new(0.5, 0.5, 1.0)];

        let c = average_samples(cv);

        assert_eq!(c, Color::new(0.25, 0.75, 0.5));
    }
}
