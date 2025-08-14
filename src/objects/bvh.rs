use crate::{
    environment::Ray,
    util::{Interval, Point3},
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// AABB stores 3 intervals to represent a binding box
/// this is used for optimization of the ray casting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }
}

impl Aabb {
    /// Takes 3 intervals to make an AABB
    pub fn new_from_intervals(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    /// Makes the bounding box from two points representing
    /// the extrema of the box
    pub fn new_from_points(a: Point3, b: Point3) -> Aabb {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };

        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };

        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };

        Aabb::new_from_intervals(x, y, z)
    }

    /// Creates a new box containing both of the parameter boxes
    pub fn new_from_boxes(box0: &Aabb, box1: &Aabb) -> Aabb {
        let x = Interval::tight_enclose(&box0.x, &box1.x);
        let y = Interval::tight_enclose(&box0.y, &box1.y);
        let z = Interval::tight_enclose(&box0.z, &box1.z);

        Aabb::new_from_intervals(x, y, z)
    }

    pub fn axis_interval(&self, n: Axis) -> &Interval {
        match n {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }

    pub fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in Axis::iter() {
            let ax = self.axis_interval(axis.clone());

            let (ray_orig_axis, ray_dir_axis) = match axis {
                Axis::X => (ray_orig.x(), ray_dir.x()),
                Axis::Y => (ray_orig.y(), ray_dir.y()),
                Axis::Z => (ray_orig.z(), ray_dir.z()),
            };
            let adinv = 1.0 / ray_dir_axis;

            let t0 = (ax.min() - ray_orig_axis) * adinv;
            let t1 = (ax.max() - ray_orig_axis) * adinv;

            let new_min;
            let new_max;

            if t0 < t1 {
                new_min = if t0 > ray_t.min() { t0 } else { ray_t.min() };
                new_max = if t1 < ray_t.max() { t1 } else { ray_t.max() };
            } else {
                new_min = if t1 > ray_t.min() { t1 } else { ray_t.min() };
                new_max = if t0 < ray_t.max() { t0 } else { ray_t.max() };
            }

            *ray_t = Interval::new(new_min, new_max);

            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }

        true
    }

    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}
