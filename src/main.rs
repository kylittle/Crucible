use clap::Parser;

use ray_tracing::{demo_scenes, environment::Camera, util::Point3};

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

    let world = demo_scenes::book1_end_scene();

    eprintln!("Creating camera with {threads} threads.");
    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 1920, threads);

    cam.set_samples(500);
    cam.set_max_depth(50);

    cam.look_from(Point3::new(13.0, 2.0, 3.0));
    cam.look_at(Point3::new(0.0, 0.0, 0.0));

    cam.set_vfov(20.0);

    cam.set_defocus_angle(0.6);
    cam.set_focus_dist(10.0);

    // cam.set_max_depth(50);
    // cam.set_samples(100);

    match cam.render(&world, args.file.as_str()) {
        Ok(()) => {
            eprintln!("Successful render! Image stored at: {}", args.file.as_str());
        }
        Err(e) => {
            eprintln!("Render failed. {e}");
        }
    }
}
