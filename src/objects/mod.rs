mod bvh;

// Reexport the creatable objects
pub mod bvhwrapper;
pub mod hitlist;
pub mod sphere;
pub mod triangle;

use crate::{
    camera::Ray,
    materials::Materials,
    objects::{
        bvh::Aabb, bvhwrapper::BVHWrapper, hitlist::HitList, sphere::Sphere, triangle::Triangle,
    },
    utils::{Interval, Point3, Vec3},
};

/// Contains information when a ray hits an object
/// the location, the surface normal, and the location
/// on the ray where the hit occured.
pub struct HitRecord {
    loc: Point3,
    normal: Vec3,
    mat: Materials,
    t: f64,
    pub u_texture: f64,
    pub v_texture: f64,
    front_face: bool,
}

impl HitRecord {
    /// Function builds a new HitRecord.
    ///
    /// # Safety
    /// This function is unsafe if the normal is not of
    /// unit length. It is not normalized here to allow
    /// math based optimizations at the geometry level.
    pub unsafe fn new(
        hit_ray: &Ray,
        loc: Point3,
        normal: Vec3,
        t: f64,
        u_texture: f64,
        v_texture: f64,
        mat: Materials,
    ) -> HitRecord {
        let front_face = hit_ray.direction().dot(&normal) < 0.0;
        let new_normal = if front_face { normal } else { -normal };

        HitRecord {
            loc,
            normal: new_normal,
            mat,
            t,
            u_texture,
            v_texture,
            front_face,
        }
    }

    /// Function that builds a safe HitRecord. This differs from
    /// the unsafe variant my making sure the normal is a unit vector
    /// this is expensive and if there are math tricks available the unsafe
    /// variant may be better
    pub fn safe_new(
        hit_ray: &Ray,
        loc: Point3,
        normal: Vec3,
        t: f64,
        u_texture: f64,
        v_texture: f64,
        mat: Materials,
    ) -> HitRecord {
        let normal = normal.unit_vector();
        let front_face = hit_ray.direction().dot(&normal) < 0.0;
        let new_normal = if front_face { normal } else { -normal };

        HitRecord {
            loc,
            normal: new_normal,
            mat,
            t,
            u_texture,
            v_texture,
            front_face,
        }
    }

    pub fn position(&self) -> Point3 {
        self.loc.clone()
    }

    pub fn normal(&self) -> Vec3 {
        self.normal.clone()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Materials {
        self.mat.clone()
    }
}

// Hittables is a wrapper around a Hittable so that there
// is no need for dyn Hittable. It also allows for each object
// to be handled specifically based on what it is
#[derive(Debug, Clone)]
pub enum Hittables {
    Sphere(Sphere),
    HitList(HitList),
    BVHWrapper(BVHWrapper),
    Triangle(Triangle),
}

impl Hittables {
    pub fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(s) => s.hit(r, ray_t),
            Hittables::HitList(l) => l.hit(r, ray_t),
            Hittables::BVHWrapper(b) => b.hit(r, ray_t),
            Hittables::Triangle(t) => t.hit(r, ray_t),
        }
    }

    pub fn bounding_box(&self) -> &Aabb {
        match self {
            Hittables::Sphere(s) => s.bounding_box(),
            Hittables::HitList(l) => l.bounding_box(),
            Hittables::BVHWrapper(b) => b.bounding_box(),
            Hittables::Triangle(t) => t.bounding_box(),
        }
    }

    /// Changes the AABB's position
    pub fn update_bb(&mut self, time: f64) {
        match self {
            Hittables::Sphere(s) => s.update_bb(time),
            Hittables::HitList(l) => l.update_bb(time),
            Hittables::BVHWrapper(_) => {}
            Hittables::Triangle(t) => t.update_bb(time),
        }
    }
}

/// An object must implement this to be rendered. This function
/// captures the hit data in rec and returns an option with some hit
/// or none.
pub trait Hittable {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}
