use std::{fmt::Debug, sync::Arc};

use nalgebra::{Matrix4, Vector4};

use crate::{
    timeline::helper_functions::{TransformResult, TransformType},
    utils::{Interval, Point3},
};

mod helper_functions;
mod matrix_builder;
mod transform_builder;

/// MatrixInfo describes a transform in time
/// the valid time interval represents the keyframes
/// for the transform while the transform_description
/// holds the time scaled transform for an item.
/// This allows for interpolated transforms at different
/// times.
struct MatrixInfo {
    transform_description: Arc<dyn Fn(f64) -> f64 + Send + Sync + 'static>,
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
    fn new<T: Fn(f64) -> f64 + Send + Sync + 'static>(transform: T) -> MatrixInfo {
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
#[derive(Debug, Clone)]
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

/// This is an argument that will be passed into relevant transforms to switch between Local and World
/// transforms. TODO: Maybe add Camera if that is useful?
#[derive(Debug, Clone)]
pub enum TransformSpace {
    World,
    Local,
}

/// TODO: I don't think any object needs to store its location after this (with a few exceptions
/// such as triangle offset vertices CHECK THIS) but some data still needs to be held like info for
/// scaling
#[derive(Debug, Clone)]
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
    pub fn new(start_pos: Point3, _start_rot: Point3, start_scale: f64) -> TransformTimeline {
        let mut scale = Vec::new();
        let rotate = Vec::new();
        let mut translate = Vec::new();

        let start_scale_mat = matrix_builder::build_other_scaler(start_scale);
        let start_mat = matrix_builder::build_pos(start_pos.clone());

        // Since scale behaves differently than the other transforms and doesn't remember its past
        // since that makes it more useable it will start with the identity scale.
        scale.push(Transform {
            transform: start_scale_mat,
            valid_time: Interval::new(-0.1, -0.1),
            transform_type: TransformType::Omni,
            start: TransformResult::InitScale(start_scale),
            end: TransformResult::InitScale(start_scale),
        });

        // Add the identity as an omni type for transform, change this for initial object rotation
        // TODO: build an initial rotation matrix so we can apply all the rotations up to a time
        // rotate.push(Transform {
        //     transform: id.clone(),
        //     valid_time: Interval::new(-0.1, -0.1),
        //     transform_type: TransformType::Omni,
        //     start: TransformResult::InitRotate(start_rot.clone()),
        //     end: TransformResult::InitRotate(start_rot.clone()),
        // });

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

    /// The new function needs to build a transform timeline and insert the frame 0 keyframes for
    /// object position:
    ///
    /// This constructor is meant specifically for spheres. Probably don't make one of these yourself
    /// and just use the scene interface.
    pub fn new_sphere(
        start_pos: Point3,
        _start_rot: Point3,
        start_radius: f64,
    ) -> TransformTimeline {
        let mut scale = Vec::new();
        let rotate = Vec::new();
        let mut translate = Vec::new();

        let start_scale_sphere = matrix_builder::build_sphere_scaler(start_radius);
        let start_mat = matrix_builder::build_pos(start_pos.clone());

        // Since scale behaves differently than the other transforms and doesn't remember its past
        // since that makes it more useable it will start with the identity scale.
        scale.push(Transform {
            transform: start_scale_sphere,
            valid_time: Interval::new(-0.1, -0.1),
            transform_type: TransformType::Omni,
            start: TransformResult::InitScale(start_radius),
            end: TransformResult::InitScale(start_radius),
        });

        // Add the identity as an omni type for transform, change this for initial object rotation
        // TODO: build an initial rotation matrix so we can apply all the rotations up to a time
        // rotate.push(Transform {
        //     transform: id.clone(),
        //     valid_time: Interval::new(-0.1, -0.1),
        //     transform_type: TransformType::Omni,
        //     start: TransformResult::InitRotate(start_rot.clone()),
        //     end: TransformResult::InitRotate(start_rot.clone()),
        // });

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
    /// TODO: I think we don't need the vector type whatsoever. Instead it will return a Point3 that tells us where to be
    /// Instead encode radius into this and treat it as a super generic way to tell position of objects. This will be a lot
    /// of changes in the Objects file
    pub fn combine_and_compute(&self, t: f64) -> Vector4<f64> {
        // Check that there are no overlap transforms TODO: Implement this

        // Get the valid matrices based on what time it is TODO: This probably shouldnt be last
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

        // TODO: Get the last XYZ scaling or the last R scaling. the type system should make these mutually exclusive
        let scale_matrix = self
            .scale
            .iter()
            .filter(|tf| tf.valid_time.is_less(t) || tf.valid_time.contains(t))
            .next_back()
            .unwrap()
            .get_matrix_at_time(t);

        // NOTE: Put ScaleR type scaling before translating, reevaluate when it comes to triangles
        let combined_matrix = scale_matrix * translate_matrix; // TODO: add rotations
        let outputs = combined_matrix * Vector4::new(0.0, 0.0, 0.0, 1.0);

        Vector4::from_row_slice(outputs.as_slice())
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
    fn check_nerp_scaling() {
        let mut timeline = TransformTimeline::new_sphere(
            Point3::new(2.0, 3.0, 0.0),
            Point3::new(2.0, 1.0, 3.0),
            1.0,
        );

        // Add a new keyframe that linearly interpolates the sphere to a radius of 10.0 after 5.0 seconds
        timeline.scale_sphere(15.0, 5.0, InterpolationType::NERP);

        // At and beyond the keyframe the value is 15.0
        let result = timeline.combine_and_compute(7.0);
        assert_eq!(result[3], 15.0);

        // before the keyframe it is the initial value
        let result = timeline.combine_and_compute(3.15);
        assert_eq!(result[3], 1.0)
    }

    #[test]
    fn check_lerp_scaling() {
        let mut timeline = TransformTimeline::new_sphere(
            Point3::new(2.0, 3.0, 0.0),
            Point3::new(2.0, 1.0, 3.0),
            1.0,
        );

        // Add a new keyframe that linearly interpolates the sphere to a radius of 10.0 after 5.0 seconds
        timeline.scale_sphere(15.0, 5.0, InterpolationType::LERP);
        timeline.scale_sphere(5.0, 10.0, InterpolationType::LERP);

        let result = timeline.combine_and_compute(5.0);
        assert_eq!(result[3], 15.0);

        let result = timeline.combine_and_compute(3.15);
        // This isnt super precise but it shows that lerp works:
        assert!((result[3] - 10.0).abs() < 0.2);
    }

    #[test]
    fn check_nerp_translate() {
        let mut timeline =
            TransformTimeline::new(Point3::new(2.0, 3.0, 1.0), Point3::origin(), 1.0);

        timeline.translate_x(1.0, 5.0, InterpolationType::NERP, TransformSpace::Local);
        timeline.translate_y(10.0, 3.0, InterpolationType::NERP, TransformSpace::Local);

        // Check that it is at its start point of 2.0
        // NOTE: the last index of the vector must be non-zero or else the equation breaks
        let result = timeline.combine_and_compute(0.0);
        assert_eq!(result[0], 2.0);
        assert_eq!(result[1], 3.0);
        // Move it when the keyframe happens
        let result = timeline.combine_and_compute(5.0);
        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 13.0);
    }
}
