use std::{fmt::Debug, sync::Arc};

use nalgebra::{Matrix4, Vector4};

use crate::{
    timeline::helper_functions::{TransformResult, TransformType},
    utils::{Interval, Point3},
};

mod helper_functions;
mod matrix_builder;

/// MatrixInfo describes a transform in time
/// the valid time interval represents the keyframes
/// for the transform while the transform_description
/// holds the time scaled transform for an item.
/// This allows for interpolated transforms at different
/// times.
struct MatrixInfo {
    transform_description: Arc<dyn Fn(f64) -> f64>,
}

impl Debug for MatrixInfo {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Clone for MatrixInfo {
    fn clone(&self) -> Self {
        MatrixInfo {
            transform_description: self.transform_description.clone(),
        }
    }
}

impl MatrixInfo {
    fn new<T: Fn(f64) -> f64 + 'static>(transform: T) -> MatrixInfo {
        MatrixInfo {
            transform_description: Arc::new(transform),
        }
    }

    /// This will give the value at interval proportion
    /// t. It is expected that the matrix clamps the value
    /// and gets the proportional distance through the
    /// interval. Otherwise this function is invalid
    ///
    /// # Safety:
    /// Make sure this is called after clamping
    /// and normalizing the input value.
    unsafe fn transform_value(&self, t: f64) -> f64 {
        self.transform_description.as_ref()(t)
    }
}

/// A transform holds the change to be applied to an
/// object, these will not be constructed directly rather
/// through the interface of the transform timeline.
/// The transform end and start are kind of like
/// links in a linked list that have to update the next transform
/// of the same type what they ended at.
#[derive(Debug)]
pub struct Transform {
    transform: Matrix4<MatrixInfo>,
    valid_time: Interval,
    transform_type: TransformType,
    start: TransformResult,
    end: TransformResult,
}

impl Transform {
    fn new(
        transform: Matrix4<MatrixInfo>,
        valid_time: Interval,
        transform_type: TransformType,
        start: TransformResult,
        end: TransformResult,
    ) -> Transform {
        Transform {
            transform,
            valid_time,
            transform_type,
            start,
            end,
        }
    }

    pub fn get_matrix_at_time(&self, t: f64) -> Matrix4<f64> {
        Matrix4::from_iterator(self.transform.iter().map(|mi| {
            let scaled_time = self.valid_time.proportion(t).clamp(0.0, 1.0);
            // Safety: The time is scaled based on the interval above
            unsafe { mi.transform_value(scaled_time) }
        }))
    }
}

/// The interpolation behavior of the keyframe. Use NERP for no interpolation.
#[derive(Debug, Clone)]
pub enum InterpolationType {
    NERP,
    LERP,
}

/// TODO: I don't think any object needs to store its location after this (with a few exceptions
/// such as triangle offset vertices CHECK THIS) but some data still needs to be held like info for
/// scaling
#[derive(Debug)]
pub struct TransformTimeline {
    scale: Vec<Transform>,
    rotate: Vec<Transform>,
    translate: Vec<Transform>,
}

impl TransformTimeline {
    /// The new function needs to build a transform timeline and insert the frame 0 keyframes for
    /// object position:
    ///
    /// Scale is a multiplier not the initial size. If for some reason you want to make an object with a
    /// radius of 3 and initialize it with a scale factor of 2x make sure to do each in the correct place
    pub fn new(start_pos: Point3, start_rot: Point3, start_scale: f64) -> TransformTimeline {
        let mut scale = Vec::new();
        let mut rotate = Vec::new();
        let mut translate = Vec::new();

        let id = matrix_builder::build_identity();
        let start_mat = matrix_builder::build_pos(start_pos.clone());

        // We need init functions to build all initial transforms

        // TODO: Evaluate if using a negative valid time is bad, this is so the initial states
        // wont interfere with adding keyframes

        // Since scale behaves differently than the other transforms and doesn't remember its past
        // since that makes it more useable it will start with the identity scale.
        scale.push(Transform {
            transform: id.clone(),
            valid_time: Interval::new(-0.1, -0.1),
            transform_type: TransformType::Omni,
            start: TransformResult::InitScale(start_scale),
            end: TransformResult::InitScale(start_scale),
        });

        // Add the identity as an omni type for transform, change this for initial object rotation
        // TODO: build an initial rotation matrix so we can apply all the rotations up to a time
        rotate.push(Transform {
            transform: id.clone(),
            valid_time: Interval::new(-0.1, -0.1),
            transform_type: TransformType::Omni,
            start: TransformResult::InitRotate(start_rot.clone()),
            end: TransformResult::InitRotate(start_rot.clone()),
        });

        // TODO: Build an initial translation matrix so we can apply all the translations up to a time
        translate.push(Transform {
            transform: start_mat,
            valid_time: Interval::new(-0.1, -0.1),
            transform_type: TransformType::Omni,
            start: TransformResult::InitTranslate(start_pos.clone()),
            end: TransformResult::InitTranslate(start_pos.clone()),
        });

        TransformTimeline {
            scale,
            rotate,
            translate,
        }
    }

    /// This function combines the transforms based on the
    /// time into a single matrix in the order Scale, Rotate,
    /// Translate
    ///
    /// The vector argument holds the data depending on the data type
    /// TODO: make it clear what this means
    pub fn combine_and_compute(&self, t: f64, inputs: Vector4<f64>) -> Vector4<f64> {
        // Check that there are no overlap transforms TODO: Implement this
        assert!(true);

        // Get the valid matrices based on what time it is TODO: This probably shouldnt be last
        let scale_test = self
            .scale
            .iter()
            .filter(|tf| tf.valid_time.is_less(t) || tf.valid_time.contains(t))
            .last()
            .unwrap();

        let translate_transforms = self
            .translate
            .iter()
            .filter(|tf| tf.valid_time.is_less(t) || tf.valid_time.contains(t))
            .map(|tf| tf.get_matrix_at_time(t));

        // Loop over and build the translate
        let mut translate_matrix = matrix_builder::build_identity_f64();
        for translate in translate_transforms {
            translate_matrix = translate * translate_matrix;
        }

        let scale_matrix = scale_test.get_matrix_at_time(t);
        // NOTE: Put ScaleR type scaling before translating, reevaluate when it comes to triangles
        let combined_matrix = scale_matrix * translate_matrix; // TODO: add rotations
        let outputs = combined_matrix * inputs;

        Vector4::from_row_slice(outputs.as_slice())
        // Then we apply the time to the dynamic matrices
        // to get a f64 matrix for each transform

        // Then we multiply the matrices together getting the
        // overall transform

        // Then we apply the transform and return the new positions
    }

    // Set Keyframe functions:
    // Note that after each keyframe is added we need to sort the Timeline by the intervals

    /// Adds a transform to the Transform timeline that changes the spheres radius.
    /// Scene will check to make sure this is applied only to valid object types.
    /// Keyframe refers to when in the animation this will occur. If interpolation
    /// is NERP this will happen instantaneously upon reaching the time keyframe.
    pub fn scale_sphere(&mut self, r: f64, keyframe: f64, interp: InterpolationType) {
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleR).expect("Missing transform data! Tried to scale radius but could not find a previous scale reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max();

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
                } else {
                    if let TransformResult::InitScale(start_scale) = prev_end {
                        scale_info = MatrixInfo::new(move |t| start_scale + (r - start_scale) * t);
                    } else {
                        panic!(
                            "Cannot find the previous scale data for radius scale at keyframe: {keyframe}"
                        )
                    }
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

    /// Translates an object along the x axis. Use this for decoupled axis movement. If you want to move an object along all three axis at the same time
    /// try `translate_point`
    pub fn translate_x(&mut self, x: f64, keyframe: f64, interp: InterpolationType) {
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateX).expect("Missing transform data! Tried to translate x but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max();

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
                    translate_info = MatrixInfo::new(move |t| start_x + (x - start_x) * t);
                } else {
                    if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                        let start_x = start_p.x();
                        translate_info = MatrixInfo::new(move |t| start_x + (x - start_x) * t);
                    } else {
                        panic!(
                            "Cannot find the previous translate data for x at keyframe: {keyframe}"
                        )
                    }
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                translate_info = MatrixInfo::new(move |_t| -> f64 { x });
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
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateY).expect("Missing transform data! Tried to translate y but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max();

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
                    translate_info = MatrixInfo::new(move |t| start_y + (y - start_y) * t);
                } else {
                    if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                        let start_y = start_p.y();
                        translate_info = MatrixInfo::new(move |t| start_y + (y - start_y) * t);
                    } else {
                        panic!(
                            "Cannot find the previous translate data for x at keyframe: {keyframe}"
                        )
                    }
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                translate_info = MatrixInfo::new(move |_t| -> f64 { y });
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
        // Gets the previous transform result
        let prev = self.most_recent_matching_transform(keyframe, TransformType::TranslateZ).expect("Missing transform data! Tried to translate z but could not find a previous position reference!");
        let prev_end = prev.end.clone();
        let prev_time = prev.valid_time.max();

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
                    translate_info = MatrixInfo::new(move |t| start_z + (z - start_z) * t);
                } else {
                    if let TransformResult::InitTranslate(start_p) = prev_end.clone() {
                        let start_z = start_p.z();
                        translate_info = MatrixInfo::new(move |t| start_z + (z - start_z) * t);
                    } else {
                        panic!(
                            "Cannot find the previous translate data for z at keyframe: {keyframe}"
                        )
                    }
                };
            }
            InterpolationType::NERP => {
                interval = Interval::new(keyframe, keyframe);
                translate_info = MatrixInfo::new(move |_t| -> f64 { z });
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_info_constant() {
        // This matrix info is a transform that moves an object instantly at 1 second into the animation
        let mi = MatrixInfo::new(|_t| 5.0);

        // As long as the time is at or past the interval we get the value
        assert_eq!(unsafe { mi.transform_value(0.0) }, 5.0);
        assert_eq!(unsafe { mi.transform_value(1.0) }, 5.0);
    }

    #[test]
    fn matrix_info_interpolation() {
        let mi = MatrixInfo::new(|t| 1.0 + (3.0 - 1.0) * t);

        // Imagine this is a scaling value. At start keyframe we want no change:
        assert_eq!(unsafe { mi.transform_value(0.0) }, 1.0);
        // Halfway through the value should be halfway scaled:
        assert_eq!(unsafe { mi.transform_value(0.5) }, 2.0);
        // Finally when we get to the end time or beyond it should reach the full scale:
        assert_eq!(unsafe { mi.transform_value(1.0) }, 3.0);
    }

    #[test]
    fn check_const_transform() {
        let mut timeline =
            TransformTimeline::new(Point3::new(2.0, 3.0, 0.0), Point3::new(2.0, 1.0, 3.0), 1.0);

        // Add a new keyframe that linearly interpolates the sphere to a radius of 10.0 after 5.0 seconds
        timeline.scale_sphere(15.0, 5.0, InterpolationType::NERP);

        // At and beyond the keyframe the value is 15.0
        let result = timeline.combine_and_compute(7.0, Vector4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(result[3], 15.0);

        // before the keyframe it is the initial value
        let result = timeline.combine_and_compute(3.15, Vector4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(result[3], 1.0)
    }

    #[test]
    fn check_lerp_scaling() {
        let mut timeline =
            TransformTimeline::new(Point3::new(2.0, 3.0, 0.0), Point3::new(2.0, 1.0, 3.0), 1.0);

        // Add a new keyframe that linearly interpolates the sphere to a radius of 10.0 after 5.0 seconds
        timeline.scale_sphere(15.0, 5.0, InterpolationType::LERP);
        timeline.scale_sphere(5.0, 10.0, InterpolationType::LERP);

        let result = timeline.combine_and_compute(5.0, Vector4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(result[3], 15.0);

        let result = timeline.combine_and_compute(3.15, Vector4::new(0.0, 0.0, 0.0, 1.0));
        // This isnt super precise but it shows that lerp works:
        assert!((result[3] - 10.0).abs() < 0.2);
    }
}
