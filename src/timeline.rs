use std::{fmt::Debug, sync::Arc};

use nalgebra::{Matrix4, Point, Scalar, Vector4};

use crate::{
    timeline::transform_matrix::Matrix,
    utils::{Interval, Point3},
};

mod transform_matrix;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// This holds info about a completed transformation.
/// This allows us to grab this info and build interpolation
/// to a next value. TODO: Think about rotations
#[derive(Debug, Clone)]
enum TransformResult {
    ScaleX(f64),
    ScaleY(f64),
    ScaleZ(f64),
    ScaleR(f64),
    Rotation(Point3),
    TranslateX(f64),
    TranslateY(f64),
    TranslateZ(f64),
}

/// Basically the same as a transform result but without the
/// data
#[derive(Debug, PartialEq)]
enum TransformType {
    ScaleX,
    ScaleY,
    ScaleZ,
    ScaleR,
    Rotate,
    TranslateX,
    TranslateY,
    TranslateZ,
}

/// A transform holds the change to be applied to an
/// object, these will not be constructed directly rather
/// through the interface of the transform timeline.
/// The transform end and start are kind of like
/// links in a linked list that have to update the next transform
/// of the same type what they ended at.
#[derive(Debug)]
struct Transform {
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
pub enum InterpolationType {
    NERP,
    LERP,
}

#[derive(Debug)]
pub struct TransformTimeline {
    scale: Vec<Transform>,
    rotate: Vec<Transform>,
    translate: Vec<Transform>,
}

impl TransformTimeline {
    /// The new function needs to build a transform timeline and insert the frame 0 keyframes for
    /// object position:
    pub fn new() -> TransformTimeline {
        let mut scale = Vec::new();
        let rotate = Vec::new();
        let translate = Vec::new();

        let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
        let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

        // Starting matrix does not change anything
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
            unit_info.clone(),
        );

        // We need init functions to build all initial transforms
        scale.push(Transform {
            transform: sm,
            valid_time: Interval::new(0.0, 0.0),
            transform_type: TransformType::ScaleR,
            start: TransformResult::ScaleR(1.0),
            end: TransformResult::ScaleR(1.0),
        });

        TransformTimeline {
            scale,
            rotate,
            translate,
        }
    }

    /// Returns the previous matching transform TODO: Refactor
    fn most_recent_matching_transform(
        &mut self,
        t: f64,
        ttype: TransformType,
    ) -> Option<&mut Transform> {
        match ttype {
            TransformType::ScaleX => {
                for transform in self.scale.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::ScaleX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleY => {
                for transform in self.scale.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::ScaleY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleZ => {
                for transform in self.scale.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::ScaleZ
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleR => {
                for transform in self.scale.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::ScaleR
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Rotate => {
                for transform in self.rotate.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::Rotate
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateX => {
                for transform in self.translate.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::TranslateX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateY => {
                for transform in self.translate.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::TranslateY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateZ => {
                for transform in self.translate.iter_mut().rev().filter(|tform| {
                    tform.valid_time.is_less(t) && tform.transform_type == TransformType::TranslateZ
                }) {
                    return Some(transform);
                }
                None
            }
        }
    }

    fn next_matching_transform(&mut self, t: f64, ttype: TransformType) -> Option<&mut Transform> {
        match ttype {
            TransformType::ScaleX => {
                for transform in self.scale.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleY => {
                for transform in self.scale.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleZ => {
                for transform in self.scale.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleZ
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleR => {
                for transform in self.scale.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleR
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Rotate => {
                for transform in self.rotate.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::Rotate
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateX => {
                for transform in self.translate.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateY => {
                for transform in self.translate.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateZ => {
                for transform in self.translate.iter_mut().filter(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateZ
                }) {
                    return Some(transform);
                }
                None
            }
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
        // Get the valid matrices based on what time it is

        let scale_test = self
            .scale
            .iter()
            .filter(|tf| tf.valid_time.is_less(t) || tf.valid_time.contains(t))
            .last()
            .unwrap();

        let time_matrix = scale_test.get_matrix_at_time(t);
        let outputs = time_matrix * inputs;

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
        let prev = self.most_recent_matching_transform(keyframe, TransformType::ScaleR).expect("Missing transform data! Tried to scale at {keyframe} but could not find a previous scale reference!");
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
                    scale_info = MatrixInfo::new(move |t| start_scale + (r - start_scale) * t)
                } else {
                    panic!("Cannot find the previous scale data for radius scale at {keyframe}")
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
        let mut timeline = TransformTimeline::new();

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
        let mut timeline = TransformTimeline::new();

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
