use std::f64::consts::PI;

use crate::{
    camera::Ray,
    materials::Materials,
    objects::{HitRecord, Hittable, bvh::Aabb},
    timeline::TransformTimeline,
    utils::{Interval, Point3, Vec3},
};

/// This object allows you to construct a sphere in the world space.
///
/// WARNING: Do not mess with the id field if this is in a
/// scene.
#[derive(Debug, Clone)]
pub struct Sphere {
    pub id: usize,
    pub hide: bool,
    pub timeline: TransformTimeline,
    mat: Materials,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Materials) -> Sphere {
        assert!(radius >= 0.0, "Cannot make a sphere with negative radius");

        // TODO: check that this is how bboxs should be built
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_from_points(center.clone() - rvec.clone(), center.clone() + rvec);

        Sphere {
            id: 0,
            hide: false,
            timeline: TransformTimeline::new_sphere(center, Point3::origin(), radius),
            mat,
            bbox,
        }
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }

    pub fn update_bb(&mut self, time: f64) {
        let sphere = self.timeline.combine_and_compute(time);
        let current_center = Point3::new(sphere[0], sphere[1], sphere[2]);
        let radius = sphere[3];

        let rvec = Vec3::new(radius, radius, radius);

        self.bbox =
            Aabb::new_from_points(current_center.clone() - rvec.clone(), current_center + rvec)
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if self.hide {
            return None;
        }

        // Calculate the position of our sphere using our timeline
        let sphere = self.timeline.combine_and_compute(r.time());
        let current_center = Point3::new(sphere[0], sphere[1], sphere[2]);
        let radius = sphere[3];

        let oc = current_center.clone() - r.origin().clone(); // (C - P) part of the circle eqn

        // Quadratic formula
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - radius.powi(2);

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
        let n = (p.clone() - current_center) / radius;

        // Calc uv for textures:
        let (u, v) = Sphere::get_sphere_uv(&n);
        // Safety: This should be safe since n is divided by the radius making it unit length
        let rec = unsafe { HitRecord::new(r, p, n, t, u, v, self.mat.clone()) };

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
