use nalgebra::Matrix4;

use crate::timeline::TransformTimeline;
use crate::{
    timeline::{
        InterpolationType, MatrixInfo, Transform,
        helper_functions::{TransformResult, TransformType},
    },
    utils::{Interval, Point3},
};

/// This impl block defines all the transforms, if you want to make a custom one implement it here
impl TransformTimeline {
    /// Adds a transform to the Transform timeline that changes the spheres radius.
    /// Scene will check to make sure this is applied only to valid object types.
    /// Keyframe refers to when in the animation this will occur. If interpolation
    /// is NERP this will happen instantaneously upon reaching the time keyframe.
    pub fn scale_sphere(&mut self, r: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a r scaling"
        );

        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleR).expect("Missing transform data! Tried to scale radius but could not find a previous scale reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::ScaleR);
        if next.is_some() {
            next.unwrap().start = TransformResult::ScaleR(r);
        }

        // TODO: check for conflicts
        let interval;
        let scale_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::ScaleR(start_scale) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_scale + (r - start_scale) * t);
                } else if let TransformResult::InitScale(start_scale) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_scale + (r - start_scale) * t);
                } else {
                    panic!(
                        "Cannot find the previous scale data for radius scale at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                scale_info = MatrixInfo::new(move |_t| -> f64 { r });
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let sm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            scale_info.clone(),
        );

        // Make sure to update where next starts from
        let scale = Transform::new(
            sm,
            interval,
            TransformType::ScaleR,
            prev_end,
            TransformResult::ScaleR(r),
        );

        self.scale.push(scale);
        // Then sort by start time
        self.scale
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Adds a transform to the Transform timeline that changes the objects x scale.
    /// Scene will check to make sure this is applied only to valid object types.
    /// Keyframe refers to when in the animation this will occur. If interpolation
    /// is NERP this will happen instantaneously upon reaching the time keyframe.
    pub fn scale_x(&mut self, x: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a x scaling"
        );

        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleX).expect("Missing transform data! Tried to scale x but could not find a previous scale reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::ScaleX);
        if next.is_some() {
            next.unwrap().start = TransformResult::ScaleX(x);
        }

        // TODO: check for conflicts
        let interval;
        let scale_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::ScaleX(start_x) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_x + (x - start_x) * t);
                } else if let TransformResult::InitScale(start_x) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_x + (x - start_x) * t);
                } else {
                    panic!(
                        "Cannot find the previous scale data for x-axis scale at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                scale_info = MatrixInfo::new(move |_t| -> f64 { x });
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let sm = Matrix4::new(
            scale_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let scale = Transform::new(
            sm,
            interval,
            TransformType::ScaleX,
            prev_end,
            TransformResult::ScaleX(x),
        );

        self.scale.push(scale);
        // Then sort by start time
        self.scale
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Adds a transform to the Transform timeline that changes the objects x scale.
    /// Scene will check to make sure this is applied only to valid object types.
    /// Keyframe refers to when in the animation this will occur. If interpolation
    /// is NERP this will happen instantaneously upon reaching the time keyframe.
    pub fn scale_y(&mut self, y: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a y scaling"
        );

        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleY).expect("Missing transform data! Tried to scale y but could not find a previous scale reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::ScaleY);
        if next.is_some() {
            next.unwrap().start = TransformResult::ScaleY(y);
        }

        // TODO: check for conflicts
        let interval;
        let scale_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::ScaleY(start_y) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_y + (y - start_y) * t);
                } else if let TransformResult::InitScale(start_y) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_y + (y - start_y) * t);
                } else {
                    panic!(
                        "Cannot find the previous scale data for y-axis scale at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                scale_info = MatrixInfo::new(move |_t| -> f64 { y });
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let sm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            scale_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let scale = Transform::new(
            sm,
            interval,
            TransformType::ScaleY,
            prev_end,
            TransformResult::ScaleY(y),
        );

        self.scale.push(scale);
        // Then sort by start time
        self.scale
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Adds a transform to the Transform timeline that changes the objects z scale.
    /// Scene will check to make sure this is applied only to valid object types.
    /// Keyframe refers to when in the animation this will occur. If interpolation
    /// is NERP this will happen instantaneously upon reaching the time keyframe.
    pub fn scale_z(&mut self, z: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a z scaling"
        );

        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleZ).expect("Missing transform data! Tried to scale x but could not find a previous scale reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::ScaleZ);
        if next.is_some() {
            next.unwrap().start = TransformResult::ScaleZ(z);
        }

        // TODO: check for conflicts
        let interval;
        let scale_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::ScaleZ(start_z) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_z + (z - start_z) * t);
                } else if let TransformResult::InitScale(start_z) = prev_end {
                    scale_info = MatrixInfo::new(move |t| start_z + (z - start_z) * t);
                } else {
                    panic!(
                        "Cannot find the previous scale data for z-axis scale at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                scale_info = MatrixInfo::new(move |_t| -> f64 { z });
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let sm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            scale_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let scale = Transform::new(
            sm,
            interval,
            TransformType::ScaleZ,
            prev_end,
            TransformResult::ScaleZ(z),
        );

        self.scale.push(scale);
        // Then sort by start time
        self.scale
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Translates an object along the x axis. Use this for decoupled axis movement. If you want to move an object along all three axis at the same time
    /// try `translate_point`
    pub fn translate_x(&mut self, x: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a x translation"
        );

        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateX).expect("Missing transform data! Tried to translate x but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::TranslateX);
        if next.is_some() {
            next.unwrap().start = TransformResult::TranslateX(x);
        }

        // TODO: check for conflicts
        let interval;
        let translate_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::TranslateX(start_x) = prev_end.clone() {
                    translate_info = MatrixInfo::new(
                        move |t| if start_x > x { -1.0 } else { 1.0 } * (start_x + (x - start_x) * t)
                    );
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_x = start_p.x();
                    translate_info = MatrixInfo::new(move |t| if start_x > x {-1.0} else {1.0} * (start_x + (x - start_x) * t));
                } else {
                    panic!(
                        "Cannot find the previous translate data for x-axis at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                if let TransformResult::TranslateX(start_x) = prev_end.clone() {
                    translate_info =
                        MatrixInfo::new(move |_t| if start_x > x { -1.0 } else { 1.0 } * x);
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_x = start_p.x();
                    translate_info =
                        MatrixInfo::new(move |_t| if start_x > x { -1.0 } else { 1.0 } * x);
                } else {
                    panic!(
                        "Cannot find the previous translate data for x-axis at keyframe: {keyframe}"
                    )
                };
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let tm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            translate_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let translate = Transform::new(
            tm,
            interval,
            TransformType::TranslateX,
            prev_end,
            TransformResult::TranslateX(x),
        );

        self.translate.push(translate);
        // Then sort by start time
        self.translate
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Translates an object along the y axis. Use this for decoupled axis movement. If you want to move an object along all three axis at the same time
    /// try `translate_point`
    pub fn translate_y(&mut self, y: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a y translation"
        );
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateY).expect("Missing transform data! Tried to translate y but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::TranslateY);
        if next.is_some() {
            next.unwrap().start = TransformResult::TranslateY(y);
        }

        // TODO: check for conflicts
        let interval;
        let translate_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::TranslateY(start_y) = prev_end.clone() {
                    translate_info = MatrixInfo::new(
                        move |t| if start_y > y { -1.0 } else { 1.0 } * (start_y + (y - start_y) * t)
                    );
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_y = start_p.y();
                    translate_info = MatrixInfo::new(move |t| if start_y > y {-1.0} else {1.0} * (start_y + (y - start_y) * t));
                } else {
                    panic!(
                        "Cannot find the previous translate data for y-axis at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                if let TransformResult::TranslateY(start_y) = prev_end.clone() {
                    translate_info =
                        MatrixInfo::new(move |_t| if start_y > y { -1.0 } else { 1.0 } * y);
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_y = start_p.y();
                    translate_info =
                        MatrixInfo::new(move |_t| if start_y > y { -1.0 } else { 1.0 } * y);
                } else {
                    panic!(
                        "Cannot find the previous translate data for y-axis at keyframe: {keyframe}"
                    )
                };
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let tm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            translate_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let translate = Transform::new(
            tm,
            interval,
            TransformType::TranslateY,
            prev_end,
            TransformResult::TranslateY(y),
        );

        self.translate.push(translate);
        // Then sort by start time
        self.translate
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Translates an object along the y axis. Use this for decoupled axis movement. If you want to move an object along all three axis at the same time
    /// try `translate_point`
    pub fn translate_z(&mut self, z: f64, keyframe: f64, interp: InterpolationType) {
        assert!(
            keyframe >= 0.0,
            "Cannot add a keyframe before the animation start. You tried to add keyframe: {keyframe} in a z translation"
        );
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateZ).expect("Missing transform data! Tried to translate z but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        // Clamp the animations into positive time
        let prev_time = prev.valid_time.max().max(0.0);

        // Gets the next transform
        let next = self.next_matching_transform(keyframe, TransformType::TranslateZ);
        if next.is_some() {
            next.unwrap().start = TransformResult::TranslateZ(z);
        }

        // TODO: check for conflicts
        let interval;
        let translate_info;
        match interp {
            InterpolationType::LERP => {
                // Note this starts immediately after the previous if you want the interpolation to be delayed
                // add another NERP keyframe that has the same scale to delay the change
                interval = Interval::new(prev_time, keyframe);

                if let TransformResult::TranslateZ(start_z) = prev_end.clone() {
                    translate_info = MatrixInfo::new(
                        move |t| if start_z > z { -1.0 } else { 1.0 } * (start_z + (z - start_z) * t)
                    );
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_z = start_p.z();
                    translate_info = MatrixInfo::new(move |t| if start_z > z {-1.0} else {1.0} * (start_z + (z - start_z) * t));
                } else {
                    panic!(
                        "Cannot find the previous translate data for z-axis at keyframe: {keyframe}"
                    )
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                if let TransformResult::TranslateZ(start_z) = prev_end.clone() {
                    translate_info =
                        MatrixInfo::new(move |_t| if start_z > z { -1.0 } else { 1.0 } * z);
                } else if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                    let start_z = start_p.z();
                    translate_info =
                        MatrixInfo::new(move |_t| if start_z > z { -1.0 } else { 1.0 } * z);
                } else {
                    panic!(
                        "Cannot find the previous translate data for z-axis at keyframe: {keyframe}"
                    )
                };
            }
        }
        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Make a non-interpolated transform matrix
        let tm = Matrix4::new(
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
            translate_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            zero_info.clone(),
            unit_info.clone(),
        );

        // Make sure to update where next starts from
        let translate = Transform::new(
            tm,
            interval,
            TransformType::TranslateZ,
            prev_end,
            TransformResult::TranslateZ(z),
        );

        self.translate.push(translate);
        // Then sort by start time
        self.translate
            .sort_by(|a, b| a.valid_time.compare_start(&b.valid_time));
    }

    /// Here is a function to translate all three axis to a point, note that you have no control over timing or individual interpolation type
    /// if you want any of those use the decoupled translations
    pub fn translate_point(&mut self, p: Point3, keyframe: f64, interp: InterpolationType) {
        self.translate_x(p.x(), keyframe, interp.clone());
        self.translate_y(p.y(), keyframe, interp.clone());
        self.translate_z(p.z(), keyframe, interp);
    }

    /// Here is a function to scale all three axis to a point, note that you have no control over timing or individual interpolation type
    /// if you want any of those use the decoupled translations
    pub fn scale_point(&mut self, p: Point3, keyframe: f64, interp: InterpolationType) {
        self.scale_x(p.x(), keyframe, interp.clone());
        self.scale_y(p.y(), keyframe, interp.clone());
        self.scale_z(p.z(), keyframe, interp);
    }
}
