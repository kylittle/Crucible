use std::collections::HashMap;

use crate::scene::{ObjectInfo, ObjectType};

/// IdVendor deals a usize value and stores the string that
/// maps to it. This allows aliasing of objects which
/// is especially important for triangle mesh maps
pub struct IdVendor {
    id_map: HashMap<String, ObjectInfo>,
    id_to_vend: usize,
}

impl IdVendor {
    /// Makes an id_vendor and sets the camera as 0, which is treated as a reserved wo
    pub fn new() -> IdVendor {
        let mut id_map = HashMap::new();
        let oi = ObjectInfo::new(0_usize, ObjectType::Camera);

        id_map.insert("cam".to_string(), oi);
        IdVendor {
            id_map,
            id_to_vend: 1,
        }
    }

    /// Returns a unique ID for an alias. Returns None
    /// if the alias is already used.
    pub fn vend_id(&mut self, alias: &str, t: ObjectType) -> Option<usize> {
        let alias = alias.to_string();

        if self.id_map.contains_key(&alias) {
            return None;
        }

        let oi = ObjectInfo::new(self.id_to_vend, t);
        self.id_map.insert(alias, oi);

        let obj_id = self.id_to_vend;
        self.id_to_vend += 1;

        Some(obj_id)
    }

    /// Looks up the id of an alias
    pub fn alias_lookup(&self, alias: &str) -> Option<ObjectInfo> {
        let alias = alias.to_string();
        self.id_map.get(&alias).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_collision() {
        let mut vendor = IdVendor::new();

        let id = vendor.vend_id("test_var", ObjectType::Sphere);
        assert!(id.is_some());

        let id2 = vendor.vend_id("test_var", ObjectType::Triangle);
        assert!(id2.is_none());
    }

    #[test]
    fn alias_coherence() {
        let mut vendor = IdVendor::new();

        let id = vendor.vend_id("test_var", ObjectType::Sphere).unwrap();
        let id2 = vendor.alias_lookup("test_var").unwrap().id;

        assert_eq!(id, id2);
    }
}
