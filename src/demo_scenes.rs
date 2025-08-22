use std::sync::Arc;

use rand::Rng;

use crate::{
    asset_loader,
    camera::Camera,
    material::{Dielectric, Lambertian, Materials, Metal},
    objects::{BVHWrapper, HitList, Hittables, Sphere},
    texture::{CheckerTexture, ImageTexture, Textures},
    util::{Color, Point3, Vec3},
};

/// Here is a function that generates the demo scene from the end of book 1
pub fn book1_end_scene(threads: usize) -> (Hittables, Camera) {
    let mut world = HitList::default();

    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    let ground_material = Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // rng to pick material
    let mut rng = rand::rng();

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
                    world.add(Hittables::Sphere(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_color_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Materials::Metal(Metal::new(albedo, fuzz));
                    world.add(Hittables::Sphere(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Materials::Dielectric(Dielectric::new(1.5));
                    world.add(Hittables::Sphere(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Materials::Dielectric(Dielectric::new(1.5));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 =
        Materials::Lambertian(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1), 1.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Materials::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 400, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(13.0, 2.0, 3.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    cam.set_defocus_angle(0.6);
    cam.set_focus_dist(10.0);

    (BVHWrapper::new_wrapper(world), cam)
    //Hittables::HitList(world)
}

/// Here is a function that generates a demo scene with moving spheres
/// TODO: make this an animation
pub fn book2_motion_blur_scene(threads: usize) -> (Hittables, Camera) {
    let mut world = HitList::default();

    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    let ground_material = Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // rng to pick material
    let mut rng = rand::rng();

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
                    let center2 = center.clone() + Vec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
                    world.add(Hittables::Sphere(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_color_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    let sphere_material = Materials::Metal(Metal::new(albedo, fuzz));
                    world.add(Hittables::Sphere(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Materials::Dielectric(Dielectric::new(1.5));
                    world.add(Hittables::Sphere(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Materials::Dielectric(Dielectric::new(1.5));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 =
        Materials::Lambertian(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1), 1.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Materials::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 400, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(13.0, 2.0, 3.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    cam.set_defocus_angle(0.6);
    cam.set_focus_dist(10.0);

    (BVHWrapper::new_wrapper(world), cam)
}

/// Demo scene for textures
pub fn checkered_spheres(threads: usize) -> (Hittables, Camera) {
    let mut world = HitList::default();

    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Materials::Lambertian(Lambertian::new_from_texture(checker.clone(), 1.0)),
    )));

    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0)),
    )));

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 400, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(13.0, 2.0, 3.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    cam.set_defocus_angle(0.6);
    cam.set_focus_dist(10.0);

    (BVHWrapper::new_wrapper(world), cam)
}

/// Demo scene for loading in .obj model files
pub fn load_teapot(threads: usize) -> (Hittables, Camera) {
    let mut world = HitList::default();

    // add the teapot
    world.add(Hittables::HitList(asset_loader::load_obj(
        "teapot.obj",
        0.5,
        Point3::new(3.0, 0.0, 0.0),
    )));

    // add the ground
    let checker = Arc::new(Textures::CheckerTexture(CheckerTexture::new_from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    let ground_material = Materials::Lambertian(Lambertian::new_from_texture(checker, 1.0));
    world.add(Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 400, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(13.0, 2.0, 3.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    cam.set_defocus_angle(0.6);
    cam.set_focus_dist(10.0);

    (BVHWrapper::new_wrapper(world), cam)
}

pub fn earth(threads: usize) -> (Hittables, Camera) {
    let mut world = HitList::default();

    let earth_texture = Textures::ImageTexture(ImageTexture::new("earthmap.jpg"));
    let earth_surface =
        Materials::Lambertian(Lambertian::new_from_texture(Arc::new(earth_texture), 1.0));
    let globe = Hittables::Sphere(Sphere::new_stationary(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    world.add(globe);

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 400, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(0.0, 0.0, 12.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    (BVHWrapper::new_wrapper(world), cam)
}
