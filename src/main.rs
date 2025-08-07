use indicatif::ProgressBar;
use ray_tracing::environment::Camera;

fn main() {
    // Image

    let image_width: u32 = 1920;
    let image_height: u32 = 1080;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    let bar = ProgressBar::new((image_height * image_width).into());
    let cam = Camera::new(16.0 / 9.0, image_width, 1.0, None);

    for j in 0..image_height {
        for i in 0..image_width {
            // decimal values for each color from 0.0 to 1.0
            let p = cam.get_pixel_pos(i, j);
            let c = cam.cast_ray(p);

            println!("{c}");

            bar.inc(1);
        }
    }

    bar.finish();
}
