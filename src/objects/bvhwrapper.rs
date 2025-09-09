use std::cmp::Ordering;

use crate::{camera::Ray, objects::{bvh::{Aabb, Axis}, hitlist::HitList, HitRecord, Hittable, Hittables}, utils::Interval};

/// Wraps hittable to allow for bounding volume hierarchy
#[derive(Debug, Clone)]
pub struct BVHWrapper {
    left: Box<Hittables>,
    right: Box<Hittables>,
    bbox: Aabb,
}

impl BVHWrapper {
    /// Builds a BVHWrapper, simply pass a list and there should be a speedup
    pub fn new_wrapper(list: HitList) -> Hittables {
        let visible_objects: Vec<Hittables> = list
            .get_objs()
            .iter()
            .filter(|o| match o {
                Hittables::BVHWrapper(_) => true,
                Hittables::HitList(_) => true,
                Hittables::Sphere(s) => !s.hide,
                Hittables::Triangle(t) => !t.hide,
            })
            .cloned()
            .collect();
        let end = visible_objects.len();
        if visible_objects.is_empty() {
            return Hittables::HitList(HitList::default());
        }
        BVHWrapper::new_from_vec(visible_objects, 0, end)
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

        let left = Box::new(left);
        let right = Box::new(right);

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
}

impl Hittable for BVHWrapper {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, &mut ray_t.clone()) {
            return None;
        }

        // Update the AABBs based on the ray's time
        // TODO: Add an AABB rotation method
        let time = r.time();
        self.left.update_bb(time);
        self.right.update_bb(time);

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
