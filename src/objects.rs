use std::{cmp::Ordering, f64::consts::PI, sync::Arc};

mod bvh;

use crate::{
    camera::Ray,
    material::Materials,
    objects::bvh::{Aabb, Axis},
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

#[derive(Debug, Clone)]
pub enum Hittables {
    Sphere(Sphere),
    HitList(HitList),
    BVHWrapper(BVHWrapper),
    Triangle(Triangle),
}

impl Hittables {
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
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
}

/// An object must implement this to be rendered. This function
/// captures the hit data in rec and returns an option with some hit
/// or none.
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}

/// The first object struct in the renderer. A sphere is
/// relatively simple and implements Hittable by solving the
/// equation x^2 + y^2 + z^2 = r^2
#[derive(Debug, Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Materials,
    bbox: Aabb,
}

impl Sphere {
    pub fn new_stationary(center: Point3, radius: f64, mat: Materials) -> Sphere {
        assert!(radius >= 0.0, "Cannot make a sphere with negative radius");

        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_from_points(center.clone() - rvec.clone(), center.clone() + rvec);

        Sphere {
            center: Ray::new(center, Point3::origin()),
            radius,
            mat,
            bbox,
        }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Materials) -> Sphere {
        assert!(radius >= 0.0, "Cannot make a sphere with negative radius");

        let mut ms = Sphere {
            center: Ray::new(center1.clone(), center2 - center1),
            radius,
            mat,
            bbox: Aabb::default(),
        };

        // TODO: Work this out so that it calculates the bbox dynamically based on time past for animations
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(
            ms.center.at(0.0) - rvec.clone(),
            ms.center.at(0.0) + rvec.clone(),
        );
        let box2 = Aabb::new_from_points(
            ms.center.at(1.0) - rvec.clone(),
            ms.center.at(1.0) + rvec.clone(),
        );

        ms.bbox = Aabb::new_from_boxes(&box1, &box2);

        ms
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = current_center.clone() - r.origin().clone(); // (C - P) part of the circle eqn

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
        let n = (p.clone() - current_center) / self.radius;

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

/// Next is a general API to store world objects
/// it also implements Hittable and handles hits for each
/// object checking them all.
#[derive(Debug, Clone)]
pub struct HitList {
    objs: Vec<Hittables>,
    bbox: Aabb,
}

impl HitList {
    pub fn new(objs: Vec<Hittables>) -> HitList {
        HitList {
            objs,
            bbox: Aabb::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objs.clear();
    }

    pub fn add(&mut self, obj: Hittables) {
        self.objs.push(obj.clone());
        self.bbox = Aabb::new_from_boxes(&self.bbox, obj.bounding_box());
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

/// Wraps hittable to allow for bounding volume hierarchy
#[derive(Debug, Clone)]
pub struct BVHWrapper {
    left: Arc<Hittables>,
    right: Arc<Hittables>,
    bbox: Aabb,
}

impl BVHWrapper {
    /// Builds a BVHWrapper, simply pass a list and there should be a speedup
    pub fn new_wrapper(list: HitList) -> Hittables {
        BVHWrapper::new_from_vec(list.objs.clone(), 0, list.objs.len())
    }

    pub fn new_from_vec(mut objects: Vec<Hittables>, start: usize, end: usize) -> Hittables {
        let bvh = BVHWrapper::help_generate(&mut objects, start, end);

        match bvh {
            Hittables::BVHWrapper(mut b) => {
                b.bbox = Aabb::new_from_boxes(b.left.bounding_box(), b.right.bounding_box());
                Hittables::BVHWrapper(b)
            }
            _ => bvh, // shouldnt get here
        }
    }

    fn help_generate(objects: &mut Vec<Hittables>, start: usize, end: usize) -> Hittables {
        let mut bbox = Aabb::default();
        for obj in objects[start..end].iter().as_ref() {
            bbox = Aabb::new_from_boxes(&bbox, obj.bounding_box());
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;

        let left;
        let right;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            let mut sub_list = objects[start..end].to_vec();
            sub_list.sort_by(|a, b| BVHWrapper::box_compare(a, b, axis.clone()));

            objects.splice(start..end, sub_list);

            let mid = start + object_span / 2;
            left = BVHWrapper::help_generate(objects, start, mid);
            right = BVHWrapper::help_generate(objects, mid, end);
        }

        let left = Arc::new(left);
        let right = Arc::new(right);

        Hittables::BVHWrapper(BVHWrapper { left, right, bbox })
    }

    fn box_compare(a: &Hittables, b: &Hittables, axis_index: Axis) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index.clone());
        let b_axis_interval = b.bounding_box().axis_interval(axis_index.clone());

        if a_axis_interval.min() < b_axis_interval.min() {
            Ordering::Less
        } else if a_axis_interval.min() > b_axis_interval.min() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    // fn box_x_compare(a: Hittables, b: Hittables) -> Ordering {
    //     BVHWrapper::box_compare(a, b, Axis::X)
    // }

    // fn box_y_compare(a: Hittables, b: Hittables) -> Ordering {
    //     BVHWrapper::box_compare(a, b, Axis::Y)
    // }

    // fn box_z_compare(a: Hittables, b: Hittables) -> Ordering {
    //     BVHWrapper::box_compare(a, b, Axis::Z)
    // }
}

impl Hittable for BVHWrapper {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, &mut ray_t.clone()) {
            return None;
        }
        let hit_left = self.left.hit(r, ray_t);

        let hit_right = self.right.hit(
            r,
            &Interval::new(
                ray_t.min(),
                if let Some(item) = &hit_left {
                    item.t
                } else {
                    ray_t.max()
                },
            ),
        );

        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

/// Fundamental building block for mesh loading.
/// TODO: We are ignoring textures for now to get shapes working nicely
#[derive(Debug, Clone)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    mat: Materials,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Materials) -> Triangle {
        let max_points = Triangle::max_points(&a, &b, &c);
        let min_points = Triangle::min_points(&a, &b, &c);

        let x_int = Interval::new(min_points.0, max_points.0);
        let y_int = Interval::new(min_points.1, max_points.1);
        let z_int = Interval::new(min_points.2, max_points.2);

        let bbox = Aabb::new_from_intervals(x_int, y_int, z_int);

        Triangle { a, b, c, mat, bbox }
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
}

impl Hittable for Triangle {
    /// Based on the Moller-Trumbore algorithm
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let e1 = self.b.clone() - self.a.clone();
        let e2 = self.c.clone() - self.a.clone();

        let ray_cross_e2 = r.direction().cross(&e2);
        let det = e1.dot(&ray_cross_e2);
        //dbg!(det);

        if det > -f64::EPSILON && det < f64::EPSILON {
            // The ray is parallel to the triangle
            return None;
        }
        let inv_det = 1.0 / det;
        let s = r.origin().clone() - self.a.clone();
        let u = inv_det * s.dot(&ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * r.direction().dot(&s_cross_e1);
        //dbg!(u);
        //dbg!(v);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Compute t to find where the intersection point occurs
        let t = inv_det * e2.dot(&s_cross_e1);
        //eprintln!("Triangle");

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
