use std::{
    fs::OpenOptions,
    io::{BufWriter, Error, Write},
    sync::Arc,
};

use dashmap::DashMap;
use indicatif::{MultiProgress, ProgressStyle};
use rand::Rng;

use crate::{
    camera::cpu_threading::ThreadInfo,
    objects::Hittables,
    scene::Skybox,
    timeline::TransformTimeline,
    utils::{Color, Degrees, Point3, Radians, Vec3},
};

mod cpu_threading;
mod miscellaneous;
mod ray_casting;
mod rendering_compute;

pub use ray_casting::Ray;

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

        Viewport {
            // These will be initialized by the camera
            viewport_height: 0.0,
            viewport_width: 0.0,
            image_height,
            image_width,
        }
    }
}

impl Clone for Viewport {
    fn clone(&self) -> Self {
        Viewport {
            viewport_height: self.viewport_height,
            viewport_width: self.viewport_width,
            image_height: self.image_height,
            image_width: self.image_width,
        }
    }
}

#[derive(Clone)]
pub enum SamplingMethod {
    Square,
}

pub struct Camera {
    // camera position
    viewport: Viewport,
    vfov: Radians,
    aspect_ratio: f64,

    // look dir
    pub look_from: TransformTimeline,
    pub look_at: TransformTimeline,
    vup: Vec3,

    // defocus fields
    defocus_angle: Radians,
    focus_dist: f64,

    // sampling
    samples: u32,
    sampling_method: SamplingMethod,
    max_depth: u32,

    // threads
    thread_count: usize,
    results: Arc<DashMap<(u32, u32), Color>>,

    // progress bars
    mp: MultiProgress,
    sty: ProgressStyle,

    // Frame info
    frame_rate: f64,
    frame: usize,
    // Shutter angle is a historical value and measured in degrees simply due to film history
    // This never gets converted to radians so its okay to leave as an f64
    shutter_angle: f64,
}

impl Camera {
    /// Builds a new camera, the camera has an aspect ratio, image_width, frame_rate, shutter_angle, and thread count
    ///
    /// frame rate and shutter_angle are locked in after construction, perhaps some of these should not be
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        frame_rate: f64,
        shutter_angle: f64,
        thread_count: usize,
    ) -> Camera {
        // Location and viewport config
        let fov = Degrees::new(90.0).as_radians();
        let h = (fov.get_angle() / 2.0).tan();
        let mut v = Viewport::new(aspect_ratio, image_width);

        // calc default focal length
        let focal_length = (Point3::new(0.0, 0.0, 0.0) - Point3::new(0.0, 0.0, -1.0)).length();

        v.viewport_height = 2.0 * h * focal_length;
        v.viewport_width = v.viewport_height * (v.image_width as f64 / v.image_height as f64);

        // Sampling presets
        let samples = 10;
        let sampling_method = SamplingMethod::Square;
        let max_depth = 10;

        let results = Arc::new(DashMap::new());
        let (mp, sty) = miscellaneous::init_pb();

        Camera {
            viewport: v,
            vfov: fov,
            aspect_ratio,

            look_from: TransformTimeline::new(Point3::origin(), Point3::origin(), 1.0),
            look_at: TransformTimeline::new(Point3::origin(), Point3::origin(), 1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: Radians::new_from_degrees(0.0),
            focus_dist: 10.0,

            samples,
            sampling_method,
            max_depth,

            thread_count,
            results,

            mp,
            sty,

            frame_rate,
            frame: 0,
            shutter_angle,
        }
    }

    pub fn next_frame(&mut self) {
        self.frame += 1
    }

    /// Returns a tuple of (width, height)
    pub fn get_res(&self) -> (usize, usize) {
        (
            self.viewport.image_width as usize,
            self.viewport.image_width as usize,
        )
    }

    /// Returns a point representing where the camera originates rays from at its current frame
    pub fn get_from_frame(&self) -> Point3 {
        // Convert to seconds
        let t = (self.frame as f64) * (1.0 / self.frame_rate);
        self.get_from(t)
    }

    /// Returns a point representing where the camera is sending rays to at its current frame
    pub fn get_at_frame(&self) -> Point3 {
        // Convert to seconds
        let t = (self.frame as f64) * (1.0 / self.frame_rate);
        self.get_at(t)
    }

    /// Sets the cameras center position in the image
    pub fn look_from(&mut self, loc: Point3) {
        // TODO: There is a bug here since the new timeline has no info about scale or rotation
        // for now neither of these are implemented on cameras so this is maybe ok, but probably needs to
        // be fixed
        self.look_from = TransformTimeline::new(loc, Point3::origin(), 1.0);

        // we have to fix the viewport
        self.fix_viewport();
    }

    /// Sets where the camera looks
    pub fn look_at(&mut self, loc: Point3) {
        self.look_at = TransformTimeline::new(loc, Point3::origin(), 1.0);

        // we have to fix the viewport
        self.fix_viewport();
    }

    pub fn set_vup(&mut self, vup: Vec3) {
        self.vup = vup;
    }

    /// Sets the vertical FOV, takes degrees and changes
    /// it automatically internally
    pub fn set_vfov(&mut self, vfov_degrees: f64) {
        self.vfov = Radians::new_from_degrees(vfov_degrees);

        self.fix_viewport();
    }

    /// Sets the FOV using the horizontal number
    pub fn set_hfov(&mut self, hfov_degrees: f64) {
        let hfov = Radians::new_from_degrees(hfov_degrees);

        // Solved equation for vfov:
        let vfov = Degrees::new_from_radians(
            (hfov.get_angle() / (2.0 * self.aspect_ratio)).tan().atan() * 2.0,
        );
        self.set_vfov(vfov.get_angle());
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

    /// Sets the cameras defocus angle, argument is in degrees
    pub fn set_defocus_angle(&mut self, da_degree: f64) {
        self.defocus_angle = Radians::new_from_degrees(da_degree);
    }

    /// Sets the cameras focus distance
    pub fn set_focus_dist(&mut self, fd: f64) {
        self.focus_dist = fd;

        self.fix_viewport();
    }

    /// Changes the number of threads a camera will render with
    pub fn set_threads(&mut self, threads: usize) {
        self.thread_count = threads;
    }

    /// This causes the camera to render an image to stdout. Note
    /// that this will truncate the file. Be careful
    ///
    /// # Error
    /// Returns an error if the file cannot be opened.
    pub fn render(&mut self, skybox: &Skybox, world: &Hittables, fname: &str) -> Result<(), Error> {
        let iw = self.viewport.image_width;
        let ih = self.viewport.image_height;

        // Make the file or truncate an existing one
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(fname)?;

        let mut bw = BufWriter::new(f);

        // Render
        let (mut threads, mut sender) = self.thread_setup(skybox, world);

        writeln!(bw, "P3\n{iw} {ih}\n255")?;

        // Dispatching jobs
        for j in 0..ih {
            for i in 0..iw {
                // decimal values for each color from 0.0 to 1.0
                let thread_info = ThreadInfo::new(i, j);

                sender.as_ref().unwrap().send(thread_info).unwrap();
            }
        }

        // Waiting for threads
        drop(sender.take());

        for thread in threads.drain(..) {
            thread.join().unwrap();
        }

        // Writing to file
        for j in 0..ih {
            for i in 0..iw {
                let color = self.results.remove(&(i, j)).unwrap().1;
                writeln!(bw, "{color}")?;
            }
        }

        bw.flush()?;
        self.mp.clear().unwrap();

        Ok(())
    }

    pub(super) fn get_from(&self, t: f64) -> Point3 {
        let shift_from = self.look_from.combine_and_compute(t);
        Point3::new(shift_from[0], shift_from[1], shift_from[2])
    }

    pub(super) fn get_at(&self, t: f64) -> Point3 {
        let shift_at = self.look_at.combine_and_compute(t);
        Point3::new(shift_at[0], shift_at[1], shift_at[2])
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera {
            // Camera position
            viewport: self.viewport.clone(),
            vfov: self.vfov.clone(),
            aspect_ratio: self.aspect_ratio,

            // Look targets
            look_from: self.look_from.clone(),
            look_at: self.look_at.clone(),
            vup: self.vup.clone(),

            // defocus vars
            defocus_angle: self.defocus_angle.clone(),
            focus_dist: self.focus_dist,

            // sampling
            samples: self.samples,
            sampling_method: self.sampling_method.clone(),
            max_depth: self.max_depth,

            // Clones have no threads
            thread_count: 0,
            results: Arc::clone(&self.results),

            mp: self.mp.clone(),
            sty: self.sty.clone(),

            frame_rate: self.frame_rate,
            frame: self.frame,
            shutter_angle: self.shutter_angle,
        }
    }
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

        let c = ray_casting::average_samples(cv);

        assert_eq!(c, Color::new(0.25, 0.75, 0.5));
    }
}
