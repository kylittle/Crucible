use criterion::{Criterion, criterion_group, criterion_main};

use std::{fs, thread};

use crucible::demo_builder::demo_images;

/// Tests the renderer using different thread counts
pub fn rendering_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Threaded Rendering");
    group.sample_size(10);

    let mut scene = demo_images::book1_end_scene(1);
    scene.scene_cam.set_samples(100);

    // 1
    group.bench_function("render one thread", |b| {
        b.iter(|| scene.render_scene("benches/criterion_bench.ppm"))
    });

    // 2
    let threads = thread::available_parallelism().unwrap().get();
    scene.scene_cam.set_threads(threads);

    group.bench_function("render sys thread", |b| {
        b.iter(|| scene.render_scene("benches/criterion_bench.ppm"))
    });

    // 3
    let threads = thread::available_parallelism().unwrap().get() / 2;
    scene.scene_cam.set_threads(threads);

    group.bench_function("render half sys thread", |b| {
        b.iter(|| scene.render_scene("benches/criterion_bench.ppm"))
    });

    // 4
    let threads = thread::available_parallelism().unwrap().get() * 2;
    scene.scene_cam.set_threads(threads);

    group.bench_function("render double sys thread", |b| {
        b.iter(|| scene.render_scene("benches/criterion_bench.ppm"))
    });

    let _ = fs::remove_file("benches/criterion_bench.ppm");

    group.finish();
}

criterion_group!(benches, rendering_benchmark);
criterion_main!(benches);
