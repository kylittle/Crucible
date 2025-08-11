use criterion::{Criterion, criterion_group, criterion_main};

use std::{fs, thread};

use ray_tracing::{
    environment::Camera,
    material::{Dielectric, Lambertian, Materials, Metal},
    objects::{HitList, Hittables, Sphere},
    util::{Color, Point3},
};

/// Tests the renderer using different thread counts
pub fn rendering_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Threaded Rendering");

    let mut world = HitList::default();

    let material_ground = Materials::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0), 0.2));
    let material_center = Materials::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5), 0.4));
    let material_left = Materials::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.2));
    let material_right = Materials::Dielectric(Dielectric::new(1.50));

    world.add(Hittables::Sphere(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Hittables::Sphere(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Hittables::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Hittables::Sphere(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let world = Hittables::HitList(world);

    // 1
    let mut cam = Camera::new(16.0 / 9.0, 1080, 1);

    group.bench_function("render one thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 2
    let threads = thread::available_parallelism().unwrap().get();
    let mut cam = Camera::new(16.0 / 9.0, 1080, threads);

    group.bench_function("render sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 3
    let threads = thread::available_parallelism().unwrap().get() / 2;
    let mut cam = Camera::new(16.0 / 9.0, 1080, threads);

    group.bench_function("render half sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 4
    let threads = thread::available_parallelism().unwrap().get() * 2;
    let mut cam = Camera::new(16.0 / 9.0, 1080, threads);

    group.bench_function("render double sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    let _ = fs::remove_file("benches/criterion_bench.ppm");

    group.finish();
}

criterion_group!(benches, rendering_benchmark);
criterion_main!(benches);
