use nalgebra::Matrix4;

use crate::{timeline::MatrixInfo, utils::Point3};

pub fn build_identity() -> Matrix4<MatrixInfo> {
    let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
    let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

    // Starting matrix does not change anything
    Matrix4::new(
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
    )
}

pub fn build_identity_f64() -> Matrix4<f64> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn build_pos(pos: Point3) -> Matrix4<MatrixInfo> {
    let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
    let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });

    let x = pos.x();
    let y = pos.y();
    let z = pos.z();

    Matrix4::new(
        unit_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        MatrixInfo::new(move |_t| x),
        zero_info.clone(),
        unit_info.clone(),
        zero_info.clone(),
        MatrixInfo::new(move |_t| y),
        zero_info.clone(),
        zero_info.clone(),
        unit_info.clone(),
        MatrixInfo::new(move |_t| z),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        unit_info.clone(),
    )
}

pub fn build_sphere_scaler(radius: f64) -> Matrix4<MatrixInfo> {
    let unit_info = MatrixInfo::new(|_t: f64| -> f64 { 1.0 });
    let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });
    let radius_info = MatrixInfo::new(move |_t: f64| -> f64 { radius });

    Matrix4::new(
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
        radius_info.clone(),
    )
}

pub fn build_other_scaler(init_scale: f64) -> Matrix4<MatrixInfo> {
    let zero_info = MatrixInfo::new(|_t: f64| -> f64 { 0.0 });
    let init_info = MatrixInfo::new(move |_t: f64| -> f64 { init_scale });

    Matrix4::new(
        init_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        init_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        init_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        zero_info.clone(),
        init_info.clone(),
    )
}
