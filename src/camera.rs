use std::{
    f64::consts::PI,
    fs::OpenOptions,
    io::{BufWriter, Error, Write},
    sync::{Arc, Mutex, RwLock, mpsc},
    thread::{self, JoinHandle},
};

use dashmap::DashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

use crate::{
    objects::Hittables,
    scene::Skybox,
    timeline::TransformTimeline,
    utils::{Color, Degrees, Interval, Point3, Radians, Vec3},
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

struct ThreadInfo {
    i: u32,
    j: u32,
}

impl ThreadInfo {
    fn new(i: u32, j: u32) -> ThreadInfo {
        ThreadInfo { i, j }
    }
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
        let (mp, sty) = init_pb();

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

    pub fn add_pb(&mut self, work: u64) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(work));
        pb.set_style(self.sty.clone());

        pb
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

    fn get_from(&self, t: f64) -> Point3 {
        let shift_from = self.look_from.combine_and_compute(t);
        Point3::new(shift_from[0], shift_from[1], shift_from[2])
    }

    fn get_at(&self, t: f64) -> Point3 {
        let shift_at = self.look_at.combine_and_compute(t);
        Point3::new(shift_at[0], shift_at[1], shift_at[2])
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

    // Call whenever any of these vars change
    fn fix_viewport(&mut self) {
        let h = (self.vfov.get_angle() / 2.0).tan();

        self.viewport.viewport_height = 2.0 * h * self.focus_dist;
        self.viewport.viewport_width = self.viewport.viewport_height
            * (self.viewport.image_width as f64 / self.viewport.image_height as f64);
    }

    // TODO: Inlined for efficiency might not work when
    // the camera can move
    /// Vector representing the horizontal viewport edge
    #[inline]
    fn viewport_u(&self, t: f64) -> Vec3 {
        self.viewport.viewport_width * self.u_basis(t)
    }

    /// Vector representing the vertical viewport edge. It is
    /// negative since the coordinate for the image are opposite
    /// to the camera (we want our vec to point down)
    #[inline]
    fn viewport_v(&self, t: f64) -> Vec3 {
        self.viewport.viewport_height * (-self.v_basis(t))
    }

    /// Subdivide the length of our viewport by pixels
    /// this gets the vector between two pixels in the
    /// x-axis.
    #[inline]
    fn pixel_delta_u(&self, t: f64) -> Vec3 {
        self.viewport_u(t) / self.viewport.image_width as f64
    }

    /// Subdivide the length of our viewport by pixels
    /// this gets the vector between two pixels in the
    /// y-axis.
    #[inline]
    fn pixel_delta_v(&self, t: f64) -> Vec3 {
        self.viewport_v(t) / self.viewport.image_height as f64
    }

    /// Compute the upper left hand corner. This uses the
    /// cameras position to move to the upper left. However
    /// the / 2.0 on the last two lines breaks generality of
    /// camera position.
    #[inline]
    fn viewport_upperleft(&self, t: f64) -> Point3 {
        let cc = self.get_from(t);
        cc - (self.focus_dist * self.w_basis(t))
            - self.viewport_u(t) / 2.0
            - self.viewport_v(t) / 2.0
    }

    #[inline]
    fn pixel_start_location(&self, t: f64) -> Point3 {
        self.viewport_upperleft(t) + 0.5 * (self.pixel_delta_u(t) + self.pixel_delta_v(t))
    }

    /// The camera can take an ij pair in the image and
    /// calculate its position relative to the camera
    fn get_pixel_pos(&self, i: u32, j: u32, offset: Point3, t: f64) -> Point3 {
        self.pixel_start_location(t)
            + ((i as f64 + offset.x()) * self.pixel_delta_u(t))
            + ((j as f64 + offset.y()) * self.pixel_delta_v(t))
    }

    #[inline]
    fn defocus_radius(&self) -> f64 {
        self.focus_dist * (self.defocus_angle.get_angle() / 2.0).tan()
    }

    // Basis vectors
    #[inline]
    fn u_basis(&self, t: f64) -> Vec3 {
        self.vup.cross(&self.w_basis(t)).unit_vector()
    }

    #[inline]
    fn v_basis(&self, t: f64) -> Vec3 {
        self.w_basis(t).cross(&self.u_basis(t))
    }

    #[inline]
    fn w_basis(&self, t: f64) -> Vec3 {
        let from = self.get_from(t);
        let at = self.get_at(t);

        (from - at).unit_vector()
    }

    #[inline]
    fn defocus_disk_u(&self, t: f64) -> Vec3 {
        self.u_basis(t) * self.defocus_radius()
    }

    #[inline]
    fn defocus_disk_v(&self, t: f64) -> Vec3 {
        self.v_basis(t) * self.defocus_radius()
    }

    // This might be repurposeable as disc sampling TODO
    fn defocus_disk_sample(&self, t: f64) -> Point3 {
        let p = Point3::random_in_unit_disk();
        let from = self.get_from(t);

        from + (p.x() * self.defocus_disk_u(t)) + (p.y() * self.defocus_disk_v(t))
    }

    fn cast_ray(
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
        let (mut threads, mut sender) = self.thread_setup_helper(skybox, world);

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

        // Let the scene advance the frame
        //self.frame += 1;

        Ok(())
    }

    fn thread_setup_helper(
        &self,
        skybox: &Skybox,
        world: &Hittables,
    ) -> (Vec<JoinHandle<()>>, Option<mpsc::Sender<ThreadInfo>>) {
        // rendering environment

        let arc_skybox = Arc::new(skybox.clone());
        let arc_cam = Arc::new(self.clone());

        // Channels
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // start threads
        let mut threads = Vec::with_capacity(self.thread_count);

        for id in 0..self.thread_count {
            // This is bad for efficiency but since each thread can have different times they all need a copy of the world
            let clone_world = world.clone();

            // Make progress bar for thread
            let work = (self.viewport.image_height * self.viewport.image_width) as u64
                / self.thread_count as u64;
            let pb = self.mp.add(ProgressBar::new(work));
            pb.set_style(self.sty.clone());

            // Start the thread
            threads.push(start_thread(
                pb,
                id,
                Arc::clone(&receiver),
                Arc::clone(&self.results),
                Arc::clone(&arc_cam),
                Arc::clone(&arc_skybox),
                Arc::new(RwLock::new(clone_world)),
            ));
        }

        (threads, Some(sender))
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
    cam: Arc<Camera>,
    skybox: Arc<Skybox>,
    world: Arc<RwLock<Hittables>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let id = id;
        let mut progress = 0;

        let cam = Box::new(cam);
        let mut world = Box::new(world);

        loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(info) => {
                    let thread_loc_i = info.i;
                    let thread_loc_j = info.j;

                    let color = cam.cast_ray(
                        thread_loc_i,
                        thread_loc_j,
                        cam.max_depth,
                        &skybox,
                        Arc::get_mut(&mut world).unwrap().get_mut().unwrap(),
                    );

                    results.insert((thread_loc_i, thread_loc_j), color);
                    if progress % 10 == 0 {
                        pb.set_message(format!("t{id}"));
                        pb.inc(10);
                    }
                    progress += 1;
                }
                Err(_) => {
                    pb.finish_and_clear();
                    break;
                }
            }
        }
    })
}

fn ray_color(r: Ray, depth: u32, sb: &Skybox, world: &mut Hittables) -> Color {
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
