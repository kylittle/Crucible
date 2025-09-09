use crate::{
    objects::{Hittables, hitlist::HitList},
    scene::{ObjectInfo, ObjectType, Scene},
    timeline::{InterpolationType, TransformSpace},
    utils::Point3,
};

/// This file has all the bindings for animating a scene.
/// These functions will type check the objects they act on
/// ensuring that the matrices are applied correctly
impl Scene {
    /// Helper for type checking and alias lookup
    fn check_and_get_alias(
        &self,
        alias: &str,
        invalid_types: &[ObjectType],
        error_msg: &str,
    ) -> ObjectInfo {
        let alias_info = self.id_vendor.alias_lookup(alias).unwrap_or_else(|| {
            panic!(
                "Could not find an object with the alias: `{alias}`. Are you sure you spelled it right?",
            )
        });

        assert!(
            !check_type(alias_info.o_type, invalid_types.to_vec()),
            "{}",
            error_msg
        );
        alias_info
    }
    // Scaling functions:

    /// Scales a scene object's x-value, this is not valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_x(&mut self, x: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        let invalid_types = [ObjectType::Sphere];
        let alias_info =
            self.check_and_get_alias(alias, &invalid_types, "ScaleX cannot apply to Spheres");

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(_) => element,
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline.scale_x(x, keyframe, it.clone());
                        t.b_timeline.scale_x(x, keyframe, it.clone());
                        t.c_timeline.scale_x(x, keyframe, it.clone());
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
        let invalid_types = [ObjectType::Sphere];
        let alias_info =
            self.check_and_get_alias(alias, &invalid_types, "ScaleY cannot apply to Spheres");

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(_) => element,
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline.scale_y(y, keyframe, it.clone());
                        t.b_timeline.scale_y(y, keyframe, it.clone());
                        t.c_timeline.scale_y(y, keyframe, it.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Scales a scene object's z-value, this is not valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_z(&mut self, z: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        let invalid_types = [ObjectType::Sphere];
        let alias_info =
            self.check_and_get_alias(alias, &invalid_types, "ScaleZ cannot apply to Spheres");

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(_) => element,
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline.scale_z(z, keyframe, it.clone());
                        t.b_timeline.scale_z(z, keyframe, it.clone());
                        t.c_timeline.scale_z(z, keyframe, it.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Scales a scene object's r-value, this is only valid on spheres
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias in not a sphere
    pub fn scale_r(&mut self, r: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        let invalid_types = [
            ObjectType::Camera,
            ObjectType::Triangle,
            ObjectType::TriangleMesh,
        ];
        let alias_info = self.check_and_get_alias(
            alias,
            &invalid_types,
            "ScaleR can only be applied to Spheres",
        );

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        s.timeline.scale_sphere(r, keyframe, it.clone());
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(_) => element,
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Scales the XYZ coordinates of a non-sphere object. Note that this couples the movement
    /// together reducing the users control over the individual axis. If you want this control
    /// use the individual axis scale functions.
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_point(&mut self, p: Point3, keyframe: f64, it: InterpolationType, alias: &str) {
        let invalid_types = [ObjectType::Sphere];
        let alias_info =
            self.check_and_get_alias(alias, &invalid_types, "ScaleAll cannot apply to Spheres");

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(_) => element,
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline.scale_point(p.clone(), keyframe, it.clone());
                        t.b_timeline.scale_point(p.clone(), keyframe, it.clone());
                        t.c_timeline.scale_point(p.clone(), keyframe, it.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Scales the XYZ coordinates uniformly with a value v. Note that this couples the movement
    /// together reducing the users control over the individual axis. If you want this control
    /// use the individual axis scale functions.
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object. Panics if the object underlying the alias is a sphere
    pub fn scale_all_uniform(&mut self, v: f64, keyframe: f64, it: InterpolationType, alias: &str) {
        self.scale_point(Point3::new(v, v, v), keyframe, it, alias);
    }

    // Rotation functions:

    // TODO: Add these

    // Translate functions:

    /// Translates a scene object's x-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_x(
        &mut self,
        x: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        let invalid_types = [];
        let alias_info = self.check_and_get_alias(
            alias,
            &invalid_types,
            "TranslateX should be able to apply to everything, open an issue please!",
        );

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        s.timeline
                            .translate_x(x, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline
                            .translate_x(x, keyframe, it.clone(), space.clone());
                        t.b_timeline
                            .translate_x(x, keyframe, it.clone(), space.clone());
                        t.c_timeline
                            .translate_x(x, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Translates a scene object's x-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_y(
        &mut self,
        y: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        let invalid_types = [];
        let alias_info = self.check_and_get_alias(
            alias,
            &invalid_types,
            "TranslateY should be able to apply to everything, open an issue please!",
        );

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        s.timeline
                            .translate_y(y, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline
                            .translate_y(y, keyframe, it.clone(), space.clone());
                        t.b_timeline
                            .translate_y(y, keyframe, it.clone(), space.clone());
                        t.c_timeline
                            .translate_y(y, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    /// Translates a scene object's z-value, this is valid on all types
    ///
    /// # Panic
    /// Panics if the alias does not have an underlying object.
    pub fn translate_z(
        &mut self,
        z: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        let invalid_types = [];
        let alias_info = self.check_and_get_alias(
            alias,
            &invalid_types,
            "TranslateZ should be able to apply to everything, open an issue please!",
        );

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        s.timeline
                            .translate_z(z, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline
                            .translate_z(z, keyframe, it.clone(), space.clone());
                        t.b_timeline
                            .translate_z(z, keyframe, it.clone(), space.clone());
                        t.c_timeline
                            .translate_z(z, keyframe, it.clone(), space.clone());
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
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
        space: TransformSpace,
        alias: &str,
    ) {
        let invalid_types = [];
        let alias_info = self.check_and_get_alias(
            alias,
            &invalid_types,
            "TranslatePoint should be able to apply to everything, open an issue please!",
        );

        // Everything is okay, find the object and add the transformation:
        let mut updated_list = HitList::default();

        for element in self.elements.get_objs().clone() {
            // Check if the element has the internal id
            let updated = match element {
                // These first cases shouldn't happen since the scenes structure is flat
                Hittables::BVHWrapper(_) => element,
                Hittables::HitList(_) => element,
                Hittables::Sphere(mut s) => {
                    if s.id == alias_info.id {
                        s.timeline
                            .translate_point(p.clone(), keyframe, it.clone(), space.clone());
                    }
                    Hittables::Sphere(s)
                }
                Hittables::Triangle(mut t) => {
                    if t.id == alias_info.id {
                        t.a_timeline.translate_point(
                            p.clone(),
                            keyframe,
                            it.clone(),
                            space.clone(),
                        );
                        t.b_timeline.translate_point(
                            p.clone(),
                            keyframe,
                            it.clone(),
                            space.clone(),
                        );
                        t.c_timeline.translate_point(
                            p.clone(),
                            keyframe,
                            it.clone(),
                            space.clone(),
                        );
                    }
                    Hittables::Triangle(t)
                }
            };
            updated_list.add(updated);
        }

        self.elements = updated_list;
    }

    // Camera operations

    /// Translates the camera's x location, alias of 'from' for the camera location, 'at' for
    /// where the ray is cast. Note if you call the cam's set_from or set_at functions
    /// from the scene directly the transform will be lost and any camera animations will have to
    /// be replaced.
    ///
    /// # Panic
    /// Panics if the alias is not 'from' or 'at'.
    pub fn cam_translate_x(
        &mut self,
        x: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        assert!(alias == "from" || alias == "at");

        if alias == "from" {
            self.scene_cam.look_from.translate_x(x, keyframe, it, space);
        } else {
            self.scene_cam.look_at.translate_x(x, keyframe, it, space);
        }
    }

    /// Translates the camera's y location, alias of 'from' for the camera location, 'at' for
    /// where the ray is cast. Note if you call the cam's set_from or set_at functions
    /// from the scene directly the transform will be lost and any camera animations will have to
    /// be replaced.
    ///
    /// # Panic
    /// Panics if the alias is not 'from' or 'at'.
    pub fn cam_translate_y(
        &mut self,
        y: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        assert!(alias == "from" || alias == "at");

        if alias == "from" {
            self.scene_cam.look_from.translate_y(y, keyframe, it, space);
        } else {
            self.scene_cam.look_at.translate_y(y, keyframe, it, space);
        }
    }

    /// Translates the camera's z location, alias of 'from' for the camera location, 'at' for
    /// where the ray is cast. Note if you call the cam's set_from or set_at functions
    /// from the scene directly the transform will be lost and any camera animations will have to
    /// be replaced.
    ///
    /// # Panic
    /// Panics if the alias is not 'from' or 'at'.
    pub fn cam_translate_z(
        &mut self,
        z: f64,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        assert!(alias == "from" || alias == "at");

        if alias == "from" {
            self.scene_cam.look_from.translate_z(z, keyframe, it, space);
        } else {
            self.scene_cam.look_at.translate_z(z, keyframe, it, space);
        }
    }

    /// Translates the camera's location, alias of 'from' for the camera location, 'at' for
    /// where the ray is cast. Note if you call the cam's set_from or set_at functions
    /// from the scene directly the transform will be lost and any camera animations will have to
    /// be replaced.
    ///
    /// # Panic
    /// Panics if the alias is not 'from' or 'at'.
    pub fn cam_translate_point(
        &mut self,
        p: Point3,
        keyframe: f64,
        it: InterpolationType,
        space: TransformSpace,
        alias: &str,
    ) {
        assert!(alias == "from" || alias == "at");

        if alias == "from" {
            self.scene_cam
                .look_from
                .translate_point(p, keyframe, it, space);
        } else {
            self.scene_cam
                .look_at
                .translate_point(p, keyframe, it, space);
        }
    }
}

fn check_type(obj_type: ObjectType, invalid_types: Vec<ObjectType>) -> bool {
    invalid_types.contains(&obj_type)
}
