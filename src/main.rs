use clap::Parser;
use crucible::demo_builder::{demo_images, demo_movies};

/// A ray-tracing renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to render to. Note: you do not need to put the file extension. This
    /// will be added for you
    #[arg(short, long)]
    file: String,
    /// How many threads to use
    #[arg(short, long)]
    threads: Option<usize>,
    /// Select a image to render. This will change later. Note that video scenes are different from images
    #[arg(short, long)]
    world: usize,
    /// Set this flag to render a movie. Check demo_movies to see what kinds of videos and change things like the framerate
    /// or shutter angle
    #[arg(short, long)]
    movie: bool,
    /// If movie is enabled then this is the duration of the movie. If you prefer you can use
    /// the frames arg instead which specifies how many frames the movie will have. If you define
    /// both the program will panic. Currently this argument requires your system to have ffmpeg on your PATH
    /// TODO: Remove ffmpeg dependencies so this can run as a standalone
    #[arg(short, long)]
    seconds: Option<f64>,
    /// Specifies the frame rate of the movies output. This is independent of the cameras framerate
    /// this defines how ffmpeg will treat the images. If the camera renders a few frames faster this will
    /// effectively allow slow motion
    #[arg(short, long)]
    rate: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let threads = args.threads.unwrap_or(std::thread::available_parallelism().expect("Cannot get the thread count of your system. Specify one when running this program.").get());

    let mut scene;

    if args.movie {
        eprintln!("Rendering a movie!");
        // Assign the movie_opts
        let frame_rate = args
            .rate
            .expect("You must provide a frame rate if you are making a movie");
        let duration = args
            .seconds
            .expect("You must provide seconds if you are making a movie");

        // Grab the scene from demo_movies
        scene = match args.world {
            1 => demo_movies::first_movie(threads, frame_rate, duration),
            2 => demo_movies::moving_teapot(threads, frame_rate, duration),
            _ => {
                eprintln!("Invalid world number. Selecting default scene");
                demo_movies::first_movie(threads, frame_rate, duration)
            }
        }
    } else {
        eprintln!("Rendering an image!");
        // Not a movie:
        // Grab the scene from demo_images
        scene = match args.world {
            1 => demo_images::book1_end_scene(threads),
            2 => demo_images::checkered_spheres(threads),
            3 => demo_images::load_teapot(threads),
            4 => demo_images::earth(threads),
            5 => demo_images::garden_skybox(threads),
            _ => {
                eprintln!("Invalid world number. Selecting default scene");
                demo_images::book1_end_scene(threads)
            }
        };
    }

    scene.render_scene(args.file.as_str());
}
