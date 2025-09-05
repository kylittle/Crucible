use std::fs;

use crate::{
    asset_loader::{self, RTWImage},
    camera::Camera,
    objects::{BVHWrapper, HitList, Hittables},
    scene::id_vendor::IdVendor,
    utils::{Color, Interval, Point3},
};

mod id_vendor;
mod movie_maker;
mod scene_animator;

/// The types of skyboxes that can be used in a scene
/// Currently only Spherical is supported.
#[derive(Debug, Clone)]
pub enum Skybox {
    Spherical(SkyboxImage),
    //Planar(SkyboxImage),
    //Triplanar(SkyboxImage),
    //CameraMapping(SkyboxImage),
    Default,
}

/// TODO: Maybe make get_color a method on a skybox and it
/// just computes it for the camera?
#[derive(Debug, Clone)]
pub struct SkyboxImage {
    image: RTWImage,
}

impl SkyboxImage {
    /// Take the uv coordinate mapping and convert it to
    /// pixel mapping in the skybox image.
    pub fn get_color(&self, u: f64, v: f64) -> Color {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;

        self.image.pixel_data(i, j)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Camera,
    Sphere,
    TriangleMesh,
    Triangle,
}

/// This struct keeps track of information about objects in the scene
#[derive(Debug, Clone, Copy)]
pub struct ObjectInfo {
    id: usize,
    o_type: ObjectType,
}

impl ObjectInfo {
    pub fn new(id: usize, o_type: ObjectType) -> ObjectInfo {
        ObjectInfo { id, o_type }
    }
}

/// A scene holds a camera, elements, a skybox, and the
/// current frame of a render. The scene will move objects
/// around if they are moveable as the time changes. This
/// can be used to render many images with small adjustments
/// for film making purposes. WARNING: Do not directly call
/// render unless you know what you are doing.
pub struct Scene {
    pub scene_cam: Camera,
    elements: HitList,
    skybox: Skybox,
    id_vendor: IdVendor,
    duration: Option<f64>,
    frame_rate: usize,
}

impl Scene {
    /// Constructor for images. Note that frame rate and
    /// shutter angle are still relevant because you may
    /// want an image with motion blur
    pub fn new_image(
        aspect_ratio: f64,
        image_width: u32,
        frame_rate: usize,
        shutter_angle: f64,
        thread_count: usize,
    ) -> Scene {
        let scene_cam = Camera::new(
            aspect_ratio,
            image_width,
            frame_rate as f64,
            shutter_angle,
            thread_count,
        );
        let elements = HitList::default();
        let skybox = Skybox::Default;

        Scene {
            scene_cam,
            elements,
            skybox,
            id_vendor: IdVendor::new(),
            duration: None,
            frame_rate,
        }
    }

    /// Constructor for movies, unlike for images this requires a
    /// duration. This will be in seconds and scene will ensure that
    /// any frame rate distortion gets accurately computed.
    pub fn new_movie(
        aspect_ratio: f64,
        image_width: u32,
        frame_rate: usize,
        shutter_angle: f64,
        thread_count: usize,
        duration: f64,
    ) -> Scene {
        let scene_cam = Camera::new(
            aspect_ratio,
            image_width,
            frame_rate as f64,
            shutter_angle,
            thread_count,
        );
        let elements = HitList::default();
        let skybox = Skybox::Default;

        Scene {
            scene_cam,
            elements,
            skybox,
            id_vendor: IdVendor::new(),
            duration: Some(duration),
            frame_rate,
        }
    }

    /// Sets the skybox to the default LERP between white
    /// and blue
    pub fn load_default_skybox(&mut self) {
        self.skybox = Skybox::Default
    }

    pub fn load_spherical_skybox(&mut self, file: &str) {
        let image = RTWImage::new(file);

        self.skybox = Skybox::Spherical(SkyboxImage { image });
    }

    /// Adds an element to the scene with a name of {alias}
    pub fn add_element(&mut self, element: Hittables, alias: &str) {
        match element {
            Hittables::BVHWrapper(_) => {
                self.elements.add(element);
            }
            Hittables::HitList(_) => {
                self.elements.add(element);
            }
            Hittables::Sphere(mut s) => {
                let internal_id = self.id_vendor.vend_id(alias, ObjectType::Sphere);
                if internal_id.is_none() {
                    panic!(
                        "This sphere's alias collides with another name in the scene! Try changing {alias} to a new name."
                    );
                }
                s.id = internal_id.unwrap();
                self.elements.add(Hittables::Sphere(s));
            }
            Hittables::Triangle(mut t) => {
                let internal_id = self.id_vendor.vend_id(alias, ObjectType::Triangle);
                if internal_id.is_none() {
                    panic!(
                        "This triangles's alias collides with another name in the scene! Try changing {alias} to a new name."
                    );
                }
                t.id = internal_id.unwrap();
                self.elements.add(Hittables::Triangle(t));
            }
        }
    }

    /// Loads an asset from an obj file, and gives it a name of {alias}
    pub fn load_asset(&mut self, asset_path: &str, alias: &str, scale: f64, shift: Point3) {
        // Check for collisions
        let internal_id = self.id_vendor.vend_id(alias, ObjectType::TriangleMesh);
        if internal_id.is_none() {
            panic!(
                "This mesh's alias collides with another name in the scene! Try changing {alias} to a new name."
            )
        }

        // Load mesh
        let triangle_mesh = asset_loader::load_obj(asset_path, scale, shift);

        // Flatten the mesh since the id keeps them associated
        for element in triangle_mesh.get_objs() {
            let element = element.clone();
            match element {
                Hittables::BVHWrapper(_) => {
                    self.elements.add(element);
                }
                Hittables::HitList(_) => {
                    self.elements.add(element);
                }
                Hittables::Sphere(mut s) => {
                    s.id = internal_id.unwrap();
                    self.elements.add(Hittables::Sphere(s));
                }
                Hittables::Triangle(mut t) => {
                    t.id = internal_id.unwrap();
                    self.elements.add(Hittables::Triangle(t));
                }
            }
        }
    }

    /// Makes an item with {alias} visible in the render
    pub fn show_element(&mut self, alias: &str) {
        self.set_visibility(alias, false);
    }

    pub fn hide_element(&mut self, alias: &str) {
        self.set_visibility(alias, true);
    }

    // TODO: Check performance here
    fn set_visibility(&mut self, alias: &str, hide: bool) {
        let mut updated_list = HitList::default();
        let internal_id = self.id_vendor.alias_lookup(alias);

        if internal_id.is_none() {
            eprintln!(
                "WARNING: The element `{alias}` does not exist. Are you sure you typed the right name?"
            );
            return;
        }

        let internal_id = internal_id.unwrap().id;

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == internal_id {
                        s.hide = hide
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == internal_id {
                        t.hide = hide
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Render scene wraps the HitList before rendering
    /// Scenes keep this unwrapped before rendering for
    /// easy alteration when working with movie type renders
    pub fn render_scene(&mut self, fname: &str) {
        if self.duration.is_some() {
            self.render_movie(fname);
        } else {
            self.render_image(fname);
        }
    }

    /// Unlike an image this creates a directory with name fname. Inside it creates a file
    /// called artifacts. This will store an image for each frame. TODO: add a delete artifacts option.
    /// After rendering each image, this will use the ffmpeg bindings in rust to put together a video with
    /// the framerate specified by the scene. TODO: Add slow and fast motion keyframing to the scene
    fn render_movie(&mut self, fname: &str) {
        fs::create_dir(fname).expect("Cannot make the movie directory");
        fs::create_dir(fname.to_owned() + "/artifacts")
            .expect("Failed to create the artifacts subdirectory");

        let frames = self.compute_frame_count();
        let digit_count = frames.to_string().len();

        let pb = self.scene_cam.add_pb(frames as u64);
        pb.set_message("starting".to_string());

        // Start rendering loop
        for frame in 0..frames {
            let image_num = format!("{frame:0>digit_count$}");
            let out_name = fname.to_owned() + "/artifacts/image" + &image_num;

            self.render_image(&out_name);
            self.scene_cam.next_frame();

            pb.set_message(format!("img{frame}"));
            pb.inc(1);
        }

        let res = self.scene_cam.get_res();
        movie_maker::make_mp4(res, self.frame_rate, digit_count, fname);
        // cleanup artifacts TODO
        // or perhaps zip it?
    }

    fn compute_frame_count(&self) -> usize {
        // TODO: fix for when the cams frame rate changes
        // For now its just a trivial conversion

        // We round up to include one more frame
        (self.duration.unwrap() * self.frame_rate as f64).ceil() as usize
    }

    fn render_image(&mut self, fname: &str) {
        let world = BVHWrapper::new_wrapper(self.elements.clone());

        // Get rid of the prints soon
        match self
            .scene_cam
            .render(&self.skybox, &world, &(fname.to_owned() + ".ppm"))
        {
            Ok(()) => {
                eprintln!("Successful render! Image stored at: {fname}.ppm");
            }
            Err(e) => {
                eprintln!("Render failed. {e}");
            }
        }
    }
}
