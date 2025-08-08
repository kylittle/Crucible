use std::{f64::INFINITY, u32};

use indicatif::ProgressBar;

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
}

impl Camera {
    /// Pass in None for the parameter camera_center to
    /// get (0, 0, 0) or specify your own center
    /// *TODO*: make sure camera center actually works and add
    /// support for camera rotations and movement
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
    ) -> Camera {
        let v = Viewport::new(aspect_ratio, image_width);
        let cc = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;

        Camera {
            viewport: v,
            focal_length,
            camera_center: cc,
        }
    }

    pub fn set_loc(&mut self, loc: Point3) {
        self.camera_center = loc;
    }

    pub fn set_focal_length(&mut self, fl: f64) {
        self.focal_length = fl;
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
    fn get_pixel_pos(&self, i: u32, j: u32) -> Point3 {
        self.pixel_start_location()
            + (i as f64 * self.pixel_delta_u())
            + (j as f64 * self.pixel_delta_v())
    }

    fn cast_ray<T: Hittable>(&self, pixel_loc: Point3, world: &T) -> Color {
        let cc = self.camera_center.clone();
        let ray_dir = pixel_loc - cc.clone();
        let ray_cast = Ray::new(cc.clone(), ray_dir);

        self.ray_color(ray_cast, world)
    }

    fn ray_color<T: Hittable>(&self, r: Ray, world: &T) -> Color {
        let hit = world.hit(&r, &Interval::new(0.0, INFINITY));
        if let Some(h) = hit {
            let norm: Vec3 = h.normal();

            // Make sure that rgb is between 0.0 and 1.0
            let r = (0.5 * (norm.x() + 1.0)).clamp(0.0, 1.0);
            let g = (0.5 * (norm.y() + 1.0)).clamp(0.0, 1.0);
            let b = (0.5 * (norm.z() + 1.0)).clamp(0.0, 1.0);

            return Color::new(r, g, b);
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
                let p = self.get_pixel_pos(i, j);
                let c = self.cast_ray(p, world);

                println!("{c}");

                bar.inc(1);
            }
        }

        bar.finish();
    }
}

// Temp

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_test() {
        let r = Ray::new(Point3::origin(), Point3::new(2.0, -3.0, 1.5));

        assert_eq!(r.at(2.0), Point3::new(4.0, -6.0, 3.0));
    }
}
