use crate::{camera::Camera, utils::{Point3, Vec3}};

impl Camera {
    // Call whenever any of these vars change
    pub(super) fn fix_viewport(&mut self) {
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
    pub(super) fn get_pixel_pos(&self, i: u32, j: u32, offset: Point3, t: f64) -> Point3 {
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
    pub(super) fn defocus_disk_sample(&self, t: f64) -> Point3 {
        let p = Point3::random_in_unit_disk();
        let from = self.get_from(t);

        from + (p.x() * self.defocus_disk_u(t)) + (p.y() * self.defocus_disk_v(t))
    }

    
}
