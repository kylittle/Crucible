use rand::Rng;

use crate::{
    material::{Dielectric, Lambertian, Materials, Metal},
    objects::{HitList, Hittables, Sphere},
    util::{Color, Point3, Vec3},
};

/// Here is a function that generates the demo scene from the end of book 1
pub fn book1_end_scene() -> Hittables {
    let mut world = HitList::default();

    let ground_material = Materials::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5), 1.0));
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
                    let sphere_material = Materials::Lambertian(Lambertian::new(albedo, 1.0));
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

    let material2 = Materials::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1), 1.0));
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

    Hittables::HitList(world)
}

/// Here is a function that generates a demo scene with moving spheres
/// TODO: make this an animation
pub fn book2_motion_blur_scene() -> Hittables {
    let mut world = HitList::default();

    let ground_material = Materials::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5), 1.0));
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
                    let sphere_material = Materials::Lambertian(Lambertian::new(albedo, 1.0));
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

    let material2 = Materials::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1), 1.0));
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

    Hittables::HitList(world)
}
