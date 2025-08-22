use criterion::{Criterion, criterion_group, criterion_main};

use std::{fs, thread};

use ray_tracing::demo_scenes;

/// Tests the renderer using different thread counts
pub fn rendering_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Threaded Rendering");
    group.sample_size(10);

    let (world, mut cam) = demo_scenes::book1_end_scene(1);

    // 1
    group.bench_function("render one thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 2
    let threads = thread::available_parallelism().unwrap().get();
    cam.set_threads(threads);

    group.bench_function("render sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 3
    let threads = thread::available_parallelism().unwrap().get() / 2;
    cam.set_threads(threads);

    group.bench_function("render half sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    // 4
    let threads = thread::available_parallelism().unwrap().get() * 2;
    cam.set_threads(threads);

    group.bench_function("render double sys thread", |b| {
        b.iter(|| cam.render(std::hint::black_box(&world), "benches/criterion_bench.ppm"))
    });

    let _ = fs::remove_file("benches/criterion_bench.ppm");

    group.finish();
}

criterion_group!(benches, rendering_benchmark);
criterion_main!(benches);
