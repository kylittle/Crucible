use std::sync::Arc;

use rand::Rng;

use crate::{
    materials::{Materials, dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    objects::{Hittables, sphere::Sphere},
    scene::Scene,
    textures::{Textures, checker_texture::CheckerTexture, image_texture::ImageTexture},
    utils::{Color, Point3},
};

/// Here is a function that generates the demo scene from the end of book 1
pub fn book1_end_scene(threads: usize) -> Scene {
    let mut b1_scene = Scene::new_image(16.0 / 9.0, 400, 24, 180.0, threads);

    b1_scene.scene_cam.set_samples(500);
    b1_scene.scene_cam.set_max_depth(50);

    b1_scene.scene_cam.look_from(Point3::new(13.0, 2.0, 3.0));
    b1_scene.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    b1_scene.scene_cam.set_vfov(20.0);

    b1_scene.scene_cam.set_defocus_angle(0.6);
    b1_scene.scene_cam.set_focus_dist(10.0);

    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    let ground_material = Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0));
    b1_scene.add_element(
        Hittables::Sphere(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )),
        "ground",
    );

    // rng to pick material
    let mut rng = rand::rng();
    let mut counter = 0;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_color() * Color::random_color();
                    let sphere_material =
                        Materials::Lambertian(Lambertian::new_from_color(albedo, 1.0));
                    b1_scene.add_element(
                        Hittables::Sphere(Sphere::new(center, 0.2, sphere_material)),
                        &("small".to_string() + &counter.to_string()),
                    );
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_color_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Materials::Metal(Metal::new(albedo, fuzz));
                    b1_scene.add_element(
                        Hittables::Sphere(Sphere::new(center, 0.2, sphere_material)),
                        &("small".to_string() + &counter.to_string()),
                    );
                } else {
                    // glass
                    let sphere_material = Materials::Dielectric(Dielectric::new(1.5));
                    b1_scene.add_element(
                        Hittables::Sphere(Sphere::new(center, 0.2, sphere_material)),
                        &("small".to_string() + &counter.to_string()),
                    );
                }
                counter += 1;
            }
        }
    }

    let material1 = Materials::Dielectric(Dielectric::new(1.5));
    b1_scene.add_element(
        Hittables::Sphere(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)),
        "large_dielectric",
    );

    let material2 =
        Materials::Lambertian(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1), 1.0));
    b1_scene.add_element(
        Hittables::Sphere(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)),
        "large_lambertian",
    );

    let material3 = Materials::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    b1_scene.add_element(
        Hittables::Sphere(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)),
        "large_metal",
    );

    b1_scene
}

/// Demo scene for textures
pub fn checkered_spheres(threads: usize) -> Scene {
    // Make cam mutable to change its behaviors
    let mut scene = Scene::new_image(16.0 / 9.0, 400, 24, 180.0, threads);

    scene.scene_cam.set_samples(500);
    scene.scene_cam.set_max_depth(50);

    scene.scene_cam.look_from(Point3::new(13.0, 2.0, 3.0));
    scene.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    scene.scene_cam.set_vfov(20.0);

    scene.scene_cam.set_defocus_angle(0.6);
    scene.scene_cam.set_focus_dist(10.0);

    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    scene.add_element(
        Hittables::Sphere(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Materials::Lambertian(Lambertian::new_from_texture(checker.clone(), 1.0)),
        )),
        "bottom_sphere",
    );

    scene.add_element(
        Hittables::Sphere(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0)),
        )),
        "top_sphere",
    );

    scene
}

/// Demo scene for loading in .obj model files
pub fn load_teapot(threads: usize) -> Scene {
    let mut teapot_scene = Scene::new_image(16.0 / 9.0, 400, 24, 180.0, threads);

    teapot_scene.scene_cam.set_samples(200);
    teapot_scene.scene_cam.set_max_depth(50);

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

    teapot_scene
}

pub fn earth(threads: usize) -> Scene {
    let mut earth_scene = Scene::new_image(16.0 / 9.0, 400, 24, 180.0, threads);

    earth_scene.scene_cam.set_samples(500);
    earth_scene.scene_cam.set_max_depth(50);

    earth_scene.scene_cam.look_from(Point3::new(0.0, 0.0, 12.0));
    earth_scene.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    earth_scene.scene_cam.set_vfov(20.0);

    let earth_texture = Textures::ImageTexture(ImageTexture::new("earthmap.jpg"));
    let earth_surface =
        Materials::Lambertian(Lambertian::new_from_texture(Arc::new(earth_texture), 1.0));
    let globe = Hittables::Sphere(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    earth_scene.add_element(globe, "earth");

    earth_scene
}

pub fn garden_skybox(threads: usize) -> Scene {
    let mut garden = Scene::new_image(16.0 / 9.0, 1920, 24, 180.0, threads);

    garden.scene_cam.set_samples(500);
    garden.scene_cam.set_max_depth(50);

    garden.scene_cam.look_from(Point3::new(0.0, 0.0, -12.0));
    garden.scene_cam.look_at(Point3::new(0.0, 0.0, 0.0));

    garden.scene_cam.set_vfov(40.0);

    let metal = Materials::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));
    let ball = Hittables::Sphere(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, metal));

    garden.add_element(ball, "metal_ball");

    garden.load_spherical_skybox("garden.hdr");

    garden
}
