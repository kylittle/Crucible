use serde::{Deserialize, Serialize};

use crate::{
    environment::Ray,
    material::Materials,
    util::{Interval, Point3, Vec3},
};

/// Contains information when a ray hits an object
/// the location, the surface normal, and the location
/// on the ray where the hit occured.
pub struct HitRecord {
    loc: Point3,
    normal: Vec3,
    mat: Materials,
    t: f64,
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
        mat: Materials,
    ) -> HitRecord {
        let front_face = hit_ray.direction().dot(&normal) < 0.0;
        let new_normal = if front_face { normal } else { -normal };

        HitRecord {
            loc,
            normal: new_normal,
            mat,
            t,
            front_face,
        }
    }

    /// Function that builds a safe HitRecord. This differs from
    /// the unsafe variant my making sure the normal is a unit vector
    /// this is expensive and if there are math tricks available the unsafe
    /// variant may be better
    pub fn safe_new<T>(
        hit_ray: &Ray,
        loc: Point3,
        normal: Vec3,
        t: f64,
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

#[derive(Serialize, Deserialize)]
pub enum Hittables {
    Sphere(Sphere),
    HitList(HitList),
}

impl Hittables {
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(s) => s.hit(r, ray_t),
            Hittables::HitList(l) => l.hit(r, ray_t),
        }
    }
}

/// An object must implement this to be rendered. This function
/// captures the hit data in rec and returns an option with some hit
/// or none.
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

/// The first object struct in the renderer. A sphere is
/// relatively simple and implements Hittable by solving the
/// equation x^2 + y^2 + z^2 = r^2
#[derive(Serialize, Deserialize)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Materials,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Materials) -> Sphere {
        assert!(radius >= 0.0, "Cannot make a sphere with negative radius");
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center.clone() - r.origin().clone(); // (C - P) part of the circle eqn

        // Quadratic formula
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None; // No hit
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a; // here is a root
        if !ray_t.surrounds(root) {
            // check if root is in acceptable range
            root = (h + sqrtd) / a; // here is the other one
            if !ray_t.surrounds(root) {
                return None; // No valid roots
            }
        }

        // We have a valid root:
        let t = root;
        let p = r.at(t);
        let n = (p.clone() - self.center.clone()) / self.radius;
        // Safety: This should be safe since n is divided by the radius making it unit length
        let rec = unsafe { HitRecord::new(r, p, n, t, self.mat.clone()) };

        Some(rec)
    }
}

/// Next is a general API to store world objects
/// it also implements Hittable and handles hits for each
/// object checking them all.
#[derive(Serialize, Deserialize)]
pub struct HitList {
    objs: Vec<Hittables>,
}

impl HitList {
    pub fn new(objs: Vec<Hittables>) -> HitList {
        HitList { objs }
    }

    pub fn clear(&mut self) {
        self.objs.clear();
    }

    pub fn add(&mut self, obj: Hittables) {
        self.objs.push(obj);
    }
}

impl Default for HitList {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest = ray_t.max();

        for obj in self.objs.as_slice() {
            let new_interval = Interval::new(ray_t.min(), closest);
            if let Some(obj) = obj.hit(r, &new_interval) {
                closest = obj.t;
                rec = Some(obj);
            }
        }

        rec
    }
}
