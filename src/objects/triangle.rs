use crate::{camera::Ray, materials::Materials, objects::{bvh::Aabb, HitRecord, Hittable}, timeline::TransformTimeline, utils::{Interval, Point3}};

/// Fundamental building block for mesh loading.
/// TODO: We are ignoring textures for now to get shapes working nicely
///
/// WARNING: Do not mess with the ID field if this is in a scene
/// TODO: make this warning no longer relevant by design
#[derive(Debug, Clone)]
pub struct Triangle {
    pub id: usize,
    pub hide: bool,
    // These will be the same but with different starting transforms
    // The animator changes these together. An advanced user could add
    // scene transforms to change these individually
    pub a_timeline: TransformTimeline,
    pub b_timeline: TransformTimeline,
    pub c_timeline: TransformTimeline,
    mat: Materials,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Materials) -> Triangle {
        let a_timeline = TransformTimeline::new(a.clone(), Point3::origin(), 1.0);
        let b_timeline = TransformTimeline::new(b.clone(), Point3::origin(), 1.0);
        let c_timeline = TransformTimeline::new(c.clone(), Point3::origin(), 1.0);

        let max_points = Triangle::max_points(&a, &b, &c);
        let min_points = Triangle::min_points(&a, &b, &c);

        let x_int = Interval::new(min_points.0, max_points.0);
        let y_int = Interval::new(min_points.1, max_points.1);
        let z_int = Interval::new(min_points.2, max_points.2);

        let bbox = Aabb::new_from_intervals(x_int, y_int, z_int);

        Triangle {
            id: 0,
            hide: false,
            a_timeline,
            b_timeline,
            c_timeline,
            mat,
            bbox,
        }
    }

    fn max_points(a: &Point3, b: &Point3, c: &Point3) -> (f64, f64, f64) {
        let x = a.x().max(b.x().max(c.x()));
        let y = a.y().max(b.y().max(c.y()));
        let z = a.z().max(b.z().max(c.z()));

        (x, y, z)
    }

    fn min_points(a: &Point3, b: &Point3, c: &Point3) -> (f64, f64, f64) {
        let x = a.x().min(b.x().min(c.x()));
        let y = a.y().min(b.y().min(c.y()));
        let z = a.z().min(b.z().min(c.z()));

        (x, y, z)
    }

    pub fn update_bb(&mut self, time: f64) {
        let a = self.a_timeline.combine_and_compute(time);
        let b = self.b_timeline.combine_and_compute(time);
        let c = self.c_timeline.combine_and_compute(time);

        let a = Point3::new(a[0], a[1], a[2]);
        let b = Point3::new(b[0], b[1], b[2]);
        let c = Point3::new(c[0], c[1], c[2]);

        let max_points = Triangle::max_points(&a, &b, &c);
        let min_points = Triangle::min_points(&a, &b, &c);

        let x_int = Interval::new(min_points.0, max_points.0);
        let y_int = Interval::new(min_points.1, max_points.1);
        let z_int = Interval::new(min_points.2, max_points.2);

        self.bbox = Aabb::new_from_intervals(x_int, y_int, z_int);
    }
}

impl Hittable for Triangle {
    /// Based on the Moller-Trumbore algorithm
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if self.hide {
            return None;
        }

        let a = self.a_timeline.combine_and_compute(r.time());
        let b = self.b_timeline.combine_and_compute(r.time());
        let c = self.c_timeline.combine_and_compute(r.time());

        let a = Point3::new(a[0], a[1], a[2]);
        let b = Point3::new(b[0], b[1], b[2]);
        let c = Point3::new(c[0], c[1], c[2]);

        let e1 = b.clone() - a.clone();
        let e2 = c.clone() - a.clone();

        let ray_cross_e2 = r.direction().cross(&e2);
        let det = e1.dot(&ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            // The ray is parallel to the triangle
            return None;
        }
        let inv_det = 1.0 / det;
        let s = r.origin().clone() - a.clone();
        let u = inv_det * s.dot(&ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * r.direction().dot(&s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Compute t to find where the intersection point occurs
        let t = inv_det * e2.dot(&s_cross_e1);

        if ray_t.surrounds(t) {
            let intersection_point = r.at(t);
            let normal = e1.cross(&e2);
            Some(HitRecord::safe_new(
                r,
                intersection_point,
                normal,
                t,
                0.0,
                0.0,
                self.mat.clone(),
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
