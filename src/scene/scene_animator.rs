use crate::{
    objects::{HitList, Hittables},
    scene::{ObjectType, Scene},
    timeline::InterpolationType,
    utils::Point3,
};

/// This file has all the bindings for animating a scene.
/// These functions will type check the objects they act on
/// ensuring that the matrices are applied correctly
impl Scene {
    // Scaling functions:

    /// Scales a scene object's x-value, this is not valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_x(&mut self, x: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![ObjectType::Sphere];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "ScaleX cannot apply to Spheres"
        );

        let mut updated_list = HitList::default();

        // Everything is okay, find the object and add the transformation:
        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        // TODO: edit the timeline
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        // TODO: edit the timeling
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Scales a scene object's y-value, this is not valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_y(&mut self, y: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![ObjectType::Sphere];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "ScaleY cannot apply to Spheres"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Scales a scene object's z-value, this is not valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_z(&mut self, z: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![ObjectType::Sphere];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "ScaleZ cannot apply to Spheres"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Scales a scene object's r-value, this is only valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias in not a sphere
    pub fn scale_r(&mut self, r: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![
            ObjectType::Camera,
            ObjectType::Triangle,
            ObjectType::TriangleMesh,
        ];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "ScaleR can only be applied to Spheres"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Scales the XYZ coordinates of a non-sphere object. Note that this couples the movement
    /// together reducing the users control over the individual axis. If you want this control
    /// use the individual axis scale functions.
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_all(&mut self, p: Point3, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![ObjectType::Sphere];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "ScaleAll cannot apply to Spheres"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Scales the XYZ coordinates uniformly with a value v. Note that this couples the movement
    /// together reducing the users control over the individual axis. If you want this control
    /// use the individual axis scale functions.
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_all_uniform(&mut self, v: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        self.scale_all(Point3::new(v, v, v), keyframe, it, alias);
    }

    // Rotation functions:

    // TODO: Add these

    // Translate functions:

    /// Translates a scene object's x-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_x(&mut self, x: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "TranslateX should be able to apply to everything, open an issue please!"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Translates a scene object's x-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_y(&mut self, y: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "TranslateY should be able to apply to everything, open an issue please!"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Translates a scene object's z-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_z(&mut self, z: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        // Start of function assertions for type checking:
        let invalid_types = vec![];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "TranslateZ should be able to apply to everything, open an issue please!"
        );

        // Everything is okay, find the object and add the transformation:
    }

    /// Translates a scene objects position based on a point, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_point(
        &mut self,
        p: Point3,
        keyframe: f64,
        it: InterpolationType,
        alias: &str,
    ) {
        // Start of function assertions for type checking:
        let invalid_types = vec![];
        let alias_info = self.id_vendor.alias_lookup(alias).expect(format!(
            "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?"
        ).as_str());
        assert!(
            !check_type(alias_info.o_type, invalid_types),
            "TranslatePoint should be able to apply to everything, open an issue please!"
        );

        // Everything is okay, find the object and add the transformation:
    }
}

fn check_type(obj_type: ObjectType, invalid_types: Vec<ObjectType>) -> bool {
    invalid_types.contains(&obj_type)
}
