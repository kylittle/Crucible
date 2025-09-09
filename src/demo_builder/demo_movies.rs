use std::sync::Arc;

use crate::{
    materials::{lambertian::Lambertian, metal::Metal, Materials}, objects::{sphere::Sphere, Hittables}, scene::Scene, textures::{checker_texture::CheckerTexture, Textures}, timeline::{InterpolationType, TransformSpace}, utils::{Color, Point3}
};

pub fn first_movie(threads: usize, frame_rate: usize, duration: f64) -> Scene {
    let mut garden = Scene::new_movie(16.0 / 9.0, 400, frame_rate, 180.0, threads, duration);

    garden.scene_cam.set_samples(50);
    garden.scene_cam.set_max_depth(5);

    garden.scene_cam.look_from(Point3::new(0.0, 0.0, -12.0));
    garden.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    garden.scene_cam.set_vfov(40.0);

    let test_ball = Materials::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));
    let ball = Hittables::Sphere(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, test_ball));

    garden.add_element(ball, "metal_ball");

    garden.load_spherical_skybox("garden.hdr");

    // Now we add some animations:
    // Rotate around the ball
    // We use world transforms since the camera needs to move around the origin
    garden.cam_translate_point(
        Point3::new(12.0, 0.0, 0.0),
        2.5,
        InterpolationType::LERP,
        TransformSpace::World,
        "from",
    );
    garden.cam_translate_point(
        Point3::new(0.0, 0.0, 12.0),
        5.0,
        InterpolationType::LERP,
        TransformSpace::World,
        "from",
    );
    garden.cam_translate_point(
        Point3::new(-12.0, 0.0, 0.0),
        7.5,
        InterpolationType::LERP,
        TransformSpace::World,
        "from",
    );
    garden.cam_translate_point(
        Point3::new(0.0, 0.0, -12.0),
        10.0,
        InterpolationType::LERP,
        TransformSpace::World,
        "from",
    );
    // Lift up and move back
    garden.cam_translate_point(
        Point3::new(0.0, 5.0, -20.0),
        15.0,
        InterpolationType::LERP,
        TransformSpace::World,
        "from",
    );

    garden
}

pub fn moving_teapot(threads: usize, frame_rate: usize, duration: f64) -> Scene {
    let mut teapot_scene = Scene::new_movie(16.0 / 9.0, 400, frame_rate, 180.0, threads, duration);

    teapot_scene.scene_cam.set_samples(50);
    teapot_scene.scene_cam.set_max_depth(5);

    teapot_scene
        .scene_cam
        .look_from(Point3::new(13.0, 2.0, 3.0));
    teapot_scene.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    teapot_scene.scene_cam.set_vfov(20.0);

    teapot_scene.scene_cam.set_defocus_angle(0.6);
    teapot_scene.scene_cam.set_focus_dist(10.0);

    let metal = Materials::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));

    // add the teapot
    teapot_scene.load_asset(
        "teapot.obj",
        "teapot",
        0.5,
        Point3::new(3.0, 0.0, 0.0),
        metal,
    );

    // add the ground
    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    let ground_material = Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0));
    teapot_scene.add_element(
        Hittables::Sphere(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )),
        "ground",
    );

    teapot_scene.translate_point(Point3::new(0.0, 0.0, -3.0), 2.5, InterpolationType::LERP, TransformSpace::Local, "teapot");

    teapot_scene
}
