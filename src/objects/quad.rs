use crate::{
    camera::Ray,
    materials::Materials,
    objects::{HitRecord, Hittable, bvh::Aabb},
    timeline::TransformTimeline,
    utils::{Interval, Point3, Vec3},
};

/// Quadrilateral object type for
#[derive(Debug, Clone)]
pub struct Quad {
    pub id: usize,
    pub hide: bool,

    /// Here are the points defining the Quad,
    /// TODO: Check to make sure quad invariants are
    /// held as the quad is transformed (this might
    /// already be fine)
    pub q: TransformTimeline,
    pub v: TransformTimeline,
    pub u: TransformTimeline,
    mat: Materials,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Materials) -> Quad {
        let bbox = Quad::get_bbox(q, u, v);
        let q = TransformTimeline::new(q, Point3::origin(), 1.0);
        let u = TransformTimeline::new(u, Point3::origin(), 1.0);
        let v = TransformTimeline::new(v, Point3::origin(), 1.0);

        Quad {
            id: 0,
            hide: false,
            q,
            v,
            u,
            mat,
            bbox,
        }
    }

    fn get_bbox(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diag1 = Aabb::new_from_points(q, q + u + v);
        let bbox_diag2 = Aabb::new_from_points(q + u, q + v);

        Aabb::new_from_boxes(&bbox_diag1, &bbox_diag2)
    }

    fn is_interior(a: f64, b: f64, rec: HitRecord) -> Option<HitRecord> {
        let unit_interval = Interval::new(0.0, 1.0);
        let mut rec = rec;

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            None
        } else {
            rec.u_texture = a;
            rec.v_texture = b;

            Some(rec)
        }
    }
}

impl Hittable for Quad {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let q = self.q.combine_and_compute(r.time());
        let u = self.u.combine_and_compute(r.time());
        let v = self.v.combine_and_compute(r.time());

        let q = Point3::new(q[0], q[1], q[2]);
        let u = Point3::new(u[0], u[1], u[2]);
        let v = Point3::new(v[0], v[1], v[2]);

        // Calculate the plane
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        // Check if the ray hits the plane
        let denom = normal.dot(r.direction());
        if denom.abs() < 1e-8 {
            // No hit if the ray is parallel
            return None;
        }

        let t = (d - normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            // Not in the interval for the hit
            return None;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - q;
        let alpha = w.dot(&planar_hitpt_vector.cross(&v));
        let beta = w.dot(&u.cross(&planar_hitpt_vector));

        // Safety: This is safe because normal has already been normalized
        let rec = unsafe { HitRecord::new(r, intersection, normal, t, 0.0, 0.0, self.mat.clone()) };

        Quad::is_interior(alpha, beta, rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn update_bb(&mut self, time: f64) {
        let q = self.q.combine_and_compute(time);
        let u = self.u.combine_and_compute(time);
        let v = self.v.combine_and_compute(time);

        let q = Point3::new(q[0], q[1], q[2]);
        let u = Point3::new(u[0], u[1], u[2]);
        let v = Point3::new(v[0], v[1], v[2]);

        self.bbox = Quad::get_bbox(q, u, v);
    }
}
