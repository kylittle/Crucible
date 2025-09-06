use crate::{
    timeline::{Transform, TransformTimeline},
    utils::Point3,
};

/// This holds info about a completed transformation.
/// This allows us to grab this info and build interpolation
/// to a next value. TODO: Think about rotations
#[derive(Debug, Clone)]
pub enum TransformResult {
    ScaleX(f64),
    ScaleY(f64),
    ScaleZ(f64),
    ScaleR(f64),
    //Rotation(Point3),
    TranslateX(f64),
    TranslateY(f64),
    TranslateZ(f64),
    InitTranslate(Point3),
    //InitRotate(Point3),
    /// We will start with a non-distorted scale
    InitScale(f64),
}

/// Basically the same as a transform result but without the
/// data
#[derive(Debug, Clone, PartialEq)]
pub enum TransformType {
    ScaleX,
    ScaleY,
    ScaleZ,
    ScaleR,
    Rotate,
    TranslateX,
    TranslateY,
    TranslateZ,
    Omni,
}

impl TransformTimeline {
    /// Returns the previous matching transform TODO: Refactor
    pub fn most_recent_matching_transform(
        &mut self,
        t: f64,
        ttype: TransformType,
    ) -> Option<&mut Transform> {
        match ttype {
            TransformType::ScaleX => {
                if let Some(transform) = self.scale.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::ScaleX
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleY => {
                if let Some(transform) = self.scale.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::ScaleY
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleZ => {
                if let Some(transform) = self.scale.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::ScaleZ
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleR => {
                if let Some(transform) = self.scale.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::ScaleR
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Rotate => {
                if let Some(transform) = self.rotate.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::Rotate
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateX => {
                if let Some(transform) = self.translate.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::TranslateX
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateY => {
                if let Some(transform) = self.translate.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::TranslateY
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateZ => {
                if let Some(transform) = self.translate.iter_mut().rev().find(|tform| {
                    tform.valid_time.is_less(t)
                        && (tform.transform_type == TransformType::TranslateZ
                            || tform.transform_type == TransformType::Omni)
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Omni => {
                panic!("This should not be able to be added as a keyframe")
            }
        }
    }

    pub fn next_matching_transform(
        &mut self,
        t: f64,
        ttype: TransformType,
    ) -> Option<&mut Transform> {
        match ttype {
            TransformType::ScaleX => {
                if let Some(transform) = self.scale.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleY => {
                if let Some(transform) = self.scale.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleZ => {
                if let Some(transform) = self.scale.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleZ
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::ScaleR => {
                if let Some(transform) = self.scale.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::ScaleR
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Rotate => {
                if let Some(transform) = self.rotate.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t) && tform.transform_type == TransformType::Rotate
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateX => {
                if let Some(transform) = self.translate.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateX
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateY => {
                if let Some(transform) = self.translate.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateY
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::TranslateZ => {
                if let Some(transform) = self.translate.iter_mut().find(|tform| {
                    tform.valid_time.is_greater(t)
                        && tform.transform_type == TransformType::TranslateZ
                }) {
                    return Some(transform);
                }
                None
            }
            TransformType::Omni => {
                panic!("This should not be able to be added as a keyframe")
            }
        }
    }
}
