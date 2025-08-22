use clap::Parser;

use ray_tracing::demo_scenes;

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
    /// Select a scene to render. This will change later
    #[arg(short, long)]
    world: usize,
}

fn main() {
    let args = Args::parse();

    let threads = args.threads.unwrap_or(std::thread::available_parallelism().expect("Cannot get the thread count of your system. Specify one when running this program.").get());

    let (world, mut cam) = match args.world {
        1 => demo_scenes::book1_end_scene(threads),
        2 => demo_scenes::book2_motion_blur_scene(threads),
        3 => demo_scenes::checkered_spheres(threads),
        4 => demo_scenes::load_teapot(threads),
        5 => demo_scenes::earth(threads),
        _ => {
            eprintln!("Invalid world number. Selecting default scene");
            demo_scenes::book1_end_scene(threads)
        }
    };

    eprintln!("Creating camera with {threads} threads.");

    match cam.render(&world, args.file.as_str()) {
        Ok(()) => {
            eprintln!("Successful render! Image stored at: {}", args.file.as_str());
        }
        Err(e) => {
            eprintln!("Render failed. {e}");
        }
    }
}
