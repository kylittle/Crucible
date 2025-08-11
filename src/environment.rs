use std::{
    fs::OpenOptions,
    io::{BufWriter, Error, Write},
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

use dashmap::DashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

use crate::{
    objects::Hittables,
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

struct ThreadInfo {
    cam: Arc<Camera>,
    i: u32,
    j: u32,

    // Serialized world
    world: Arc<Vec<u8>>,
}

impl ThreadInfo {
    fn new(cam: Arc<Camera>, i: u32, j: u32, world: Arc<Vec<u8>>) -> ThreadInfo {
        ThreadInfo { cam, i, j, world }
    }
}
pub struct Camera {
    viewport: Viewport,
    focal_length: f64,
    camera_center: Point3,
    samples: u32,
    sampling_method: SamplingMethod,
    max_depth: u32,

    // threads
    thread_count: usize,
    render_threads: Vec<JoinHandle<()>>,
    results: Arc<DashMap<(u32, u32), Color>>,
    sender: Option<mpsc::Sender<ThreadInfo>>,

    // progress bars
    mp: MultiProgress,
    sty: ProgressStyle,
}

impl Camera {
    /// Pass in None for the parameter camera_center to
    /// get (0, 0, 0) or specify your own center
    /// *TODO*: make sure camera center actually works and add
    /// support for camera rotations and movement
    pub fn new(aspect_ratio: f64, image_width: u32, thread_count: usize) -> Camera {
        let v = Viewport::new(aspect_ratio, image_width);
        let cc = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let samples = 10;
        let sampling_method = SamplingMethod::Square;
        let max_depth = 10;

        assert!(thread_count > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let results = Arc::new(DashMap::new());

        let (mp, sty) = init_pb();

        let mut threads = Vec::with_capacity(thread_count);
        for id in 0..thread_count {
            let work = (v.image_height * v.image_width) as u64 / thread_count as u64;
            let pb = mp.add(ProgressBar::new(work));
            pb.set_style(sty.clone());
            threads.push(start_thread(
                pb,
                id,
                Arc::clone(&receiver),
                Arc::clone(&results),
            ));
        }

        Camera {
            viewport: v,
            focal_length,
            camera_center: cc,
            samples,
            sampling_method,
            max_depth,

            thread_count,
            render_threads: threads,
            results,
            sender: Some(sender),

            mp,
            sty,
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

    fn cast_ray(&self, render_i: u32, render_j: u32, max_depth: u32, world: &Hittables) -> Color {
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

            sample_colors.push(ray_color(ray_cast, max_depth, world));
        }

        average_samples(sample_colors)
    }

    /// This causes the camera to render an image to stdout. Note
    /// that this will truncate the file. Be careful
    ///
    /// # Error
    /// Returns an error if the file cannot be opened.
    pub fn render(&mut self, world: &Hittables, fname: &str) -> Result<(), Error> {
        assert!(
            self.sender.is_some(),
            "Camera is not ready, prepare it using prepare_cam()"
        );

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

        writeln!(bw, "P3\n{iw} {ih}\n255")?;

        let ser_world = Arc::new(postcard::to_allocvec(&world).unwrap());
        let arc_cam = Arc::new(self.clone());

        for j in 0..ih {
            for i in 0..iw {
                // decimal values for each color from 0.0 to 1.0
                let thread_info =
                    ThreadInfo::new(Arc::clone(&arc_cam), i, j, Arc::clone(&ser_world));

                self.sender.as_ref().unwrap().send(thread_info).unwrap();
            }
        }

        drop(self.sender.take());

        for thread in self.render_threads.drain(..) {
            thread.join().unwrap();
        }

        for j in 0..ih {
            for i in 0..iw {
                let color = self.results.remove(&(i, j)).unwrap().1;
                writeln!(bw, "{color}")?;
            }
        }

        bw.flush()?;
        self.mp.clear().unwrap();

        self.prepare_cam();

        Ok(())
    }

    fn prepare_cam(&mut self) {
        assert!(self.thread_count > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let results = Arc::new(DashMap::new());

        let mut threads = Vec::with_capacity(self.thread_count);
        self.sender = Some(sender);

        for id in 0..self.thread_count {
            let work = (self.viewport.image_height * self.viewport.image_width) as u64
                / self.thread_count as u64;
            let pb = self.mp.add(ProgressBar::new(work));
            pb.set_style(self.sty.clone());

            threads.push(start_thread(
                pb,
                id,
                Arc::clone(&receiver),
                Arc::clone(&results),
            ));
        }

        self.render_threads = threads;
        self.results = results;
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera {
            viewport: self.viewport.clone(),
            focal_length: self.focal_length,
            camera_center: self.camera_center.clone(),
            samples: self.samples,
            sampling_method: self.sampling_method.clone(),
            max_depth: self.max_depth,

            // Clones have no threads
            thread_count: 0,
            render_threads: vec![],
            results: Arc::clone(&self.results),
            sender: None,

            mp: self.mp.clone(),
            sty: self.sty.clone(),
        }
    }
}

// Helper functions
fn init_pb() -> (MultiProgress, ProgressStyle) {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    (m, sty)
}

fn start_thread(
    pb: ProgressBar,
    id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<ThreadInfo>>>,
    results: Arc<DashMap<(u32, u32), Color>>,
) -> JoinHandle<()> {
    //let pb = mp.add(ProgressBar::new(my_pixels));
    //pb.set_style(sty.clone());

    thread::spawn(move || {
        let id = id;
        let mut progress = 0;

        let mut world: Option<Hittables> = None;

        loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(info) => {
                    let cam = info.cam;

                    let thread_loc_i = info.i;
                    let thread_loc_j = info.j;

                    world = if world.is_none() {
                        postcard::from_bytes(&info.world).ok()
                    } else {
                        world
                    };

                    let w = world.as_ref().unwrap();
                    let color = cam.cast_ray(thread_loc_i, thread_loc_j, cam.max_depth, &w);

                    results.insert((thread_loc_i, thread_loc_j), color);
                    if progress % 1000 == 0 {
                        pb.set_message(format!("t{}", id));
                        pb.inc(1000);
                    }
                    progress += 1;
                }
                Err(_) => {
                    pb.finish_with_message(format!("t{} done", { id }));
                    break;
                }
            }
        }
    })
}

fn ray_color(r: Ray, depth: u32, world: &Hittables) -> Color {
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
            return attenuation * ray_color(s, depth - 1, world);
        }

        return Color::black();
    }

    let unit_direction = r.direction().clone().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
}

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
