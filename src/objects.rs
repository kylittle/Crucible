use std::rc::Rc;

use crate::{
    environment::Ray,
    util::{Point3, Vec3},
};

/// Contains information when a ray hits an object
/// the location, the surface normal, and the location
/// on the ray where the hit occured.
#[derive(Debug)]
pub struct HitRecord {
    loc: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    /// Function builds a new HitRecord.
    ///
    /// #Safety:
    /// This function is unsafe if the normal is not of
    /// unit length. It is not normalized here to allow
    /// math based optimizations at the geometry level.
    pub unsafe fn new(hit_ray: &Ray, loc: Point3, normal: Vec3, t: f64) -> HitRecord {
        let front_face = hit_ray.direction().dot(&normal) < 0.0;
        let new_normal = if front_face { normal } else { -normal };

        HitRecord {
            loc,
            normal: new_normal,
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

}

/// An object must implement this to be rendered. This function
/// captures the hit data in rec and returns an option with some hit
/// or none.
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

/// The first object struct in the renderer. A sphere is
/// relatively simple and implements Hittable by solving the
/// equation x^2 + y^2 + z^2 = r^2
#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        assert!(radius >= 0.0, "Cannot make a sphere with negative radius");
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if root <= ray_tmin || ray_tmax <= root {
            // check if root is in acceptable range
            root = (h + sqrtd) / a; // here is the other one
            if root <= ray_tmin || ray_tmax <= root {
                return None; // No valid roots
            }
        }

        // We have a valid root:
        let t = root;
        let p = r.at(t);
        let n = (p.clone() - self.center.clone()) / self.radius;
        // Safety: This should be safe since n is divided by the radius making it unit length
        let rec = unsafe { HitRecord::new(r, p, n, t) };

        Some(rec)
    }
}

/// Next is a general API to store world objects
/// it also implements Hittable and handles hits for each
/// object checking them all.
pub struct HitList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T>(&mut self, obj: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Rc::new(obj));
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest = ray_tmax;

        for obj in self.objects.as_slice() {
            if let Some(obj) = obj.hit(r, ray_tmin, closest) {
                closest = obj.t;
                rec = Some(obj);
            }
        }

        rec
    }
}
