use crate::{camera::Ray, objects::{bvh::Aabb, HitRecord, Hittable, Hittables}, utils::Interval};

/// This is a general API to store world objects
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

    pub fn get_objs(&self) -> &Vec<Hittables> {
        &self.objs
    }

    pub fn update_bb(&mut self, time: f64) {
        let mut bbox = Aabb::default();

        for obj in self.objs.iter_mut() {
            obj.update_bb(time);
            bbox = Aabb::new_from_boxes(&bbox, obj.bounding_box());
        }

        self.bbox = bbox;
    }
}

impl Default for HitList {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Hittable for HitList {
    fn hit(&mut self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest = ray_t.max();

        for obj in self.objs.as_mut_slice() {
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
