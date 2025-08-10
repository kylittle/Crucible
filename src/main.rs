use clap::Parser;
use std::rc::Rc;

use ray_tracing::{
    environment::Camera,
    material::{Lambertian, Metal},
    objects::{HitList, Sphere},
    util::{Color, Point3},
};

/// A ray-tracing renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to render to
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut world = HitList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0), 0.2));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5), 0.4));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    // Make cam mutable to change its behaviors
    let mut cam = Camera::new(16.0 / 9.0, 1080);

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
