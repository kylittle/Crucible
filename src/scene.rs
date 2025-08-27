use crate::{
    asset_loader::{self, RTWImage},
    camera::Camera,
    objects::{BVHWrapper, HitList, Hittables},
    scene::id_vendor::IdVendor,
    utils::{Color, Interval, Point3},
};

mod id_vendor;

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
    elements: HitList,
    skybox: Skybox,
    frame: u32,
    id_vendor: IdVendor,
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
            id_vendor: IdVendor::new(),
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
                let internal_id = self.id_vendor.vend_id(alias);
                if internal_id.is_none() {
                    panic!(
                        "This sphere's alias collides with another name in the scene! Try changing {alias} to a new name."
                    );
                }
                s.id = internal_id.unwrap();
                self.elements.add(Hittables::Sphere(s));
            }
            Hittables::Triangle(mut t) => {
                let internal_id = self.id_vendor.vend_id(alias);
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
        let internal_id = self.id_vendor.vend_id(alias);
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

        let internal_id = internal_id.unwrap();

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
