use clap::Parser;

use ray_tracing::{
    environment::Camera,
    material::{Lambertian, Materials, Metal},
    objects::{HitList, Hittables, Sphere},
    util::{Color, Point3},
};

/// A ray-tracing renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to render to
    #[arg(short, long)]
    file: String,
    /// How many threads to use
    #[arg(short, long)]
    threads: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let threads = args.threads.unwrap_or(std::thread::available_parallelism().expect("Cannot get the thread count of your system. Specify one when running this program.").get());

    let mut world = HitList::default();

    // Modify the add method to auto wrap in the enum.
    // There is a lot of code gen needed here, but
    // not gonna invest until its clearly worth it with
    // benchmarks
    let material_ground = Materials::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0), 0.2));
    let material_center = Materials::Lambertian(Lambertian::new(Color::new(0.1, 0.2, 0.5), 0.4));
    let material_left = Materials::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Materials::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

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

    eprintln!("Creating camera with {threads} threads.");
    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 1920, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    match cam.render(&world, args.file.as_str()) {
        Ok(()) => {
            eprintln!("Successful render! Image stored at: {}", args.file.as_str());
        }
        Err(e) => {
            eprintln!("Render failed. {e}");
        }
    }
}
