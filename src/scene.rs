use crate::{
    asset_loader::RTWImage,
    camera::Camera,
    objects::{BVHWrapper, HitList},
    util::{Color, Interval},
};

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

/// A scene holds a camera, elements, a skybox, and the
/// current frame of a render. The scene will move objects
/// around if they are moveable as the time changes. This
/// can be used to render many images with small adjustments
/// for film making purposes. WARNING: Do not directly call
/// render unless you know what you are doing.
pub struct Scene {
    pub scene_cam: Camera,
    pub elements: HitList,
    skybox: Skybox,
    frame: u32,
}

impl Scene {
    pub fn new(aspect_ratio: f64, image_width: u32, thread_count: usize) -> Scene {
        let scene_cam = Camera::new(aspect_ratio, image_width, thread_count);
        let elements = HitList::default();
        let skybox = Skybox::Default;

        Scene {
            scene_cam,
            elements,
            skybox,
            frame: 0,
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

    /// Advances the scene forward one frame, when
    /// render scene is called this value will augment where
    /// the objects are. TODO consider frame rates
    pub fn advance_frame(&mut self) {
        self.frame += 1;
    }

    /// Render scene wraps the HitList before rendering
    /// Scenes keep this unwrapped before rendering for
    /// easy alteration when working with movie type renders
    pub fn render_scene(&self, fname: &str) {
        let world = BVHWrapper::new_wrapper(self.elements.clone());

        // Get rid of the prints soon
        match self.scene_cam.render(&self.skybox, &world, fname) {
            Ok(()) => {
                eprintln!("Successful render! Image stored at: {fname}");
            }
            Err(e) => {
                eprintln!("Render failed. {e}");
            }
        }
    }
}
