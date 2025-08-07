use indicatif::ProgressBar;
use ray_tracing::math_tools::*;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    let bar = ProgressBar::new(image_height * image_width);

    for j in 0..image_height {
        for i in 0..image_width {
            // decimal values for each color from 0.0 to 1.0
            let i_decimal = i as f64;
            let j_decimal = j as f64;

            let h = (image_height - 1) as f64;
            let w = (image_width - 1) as f64;

            let pixel_color = Color::new(i_decimal / w, j_decimal / h, 0.0);
            println!("{}", -pixel_color * 0.5);

            bar.inc(1);
        }
    }

    bar.finish();
}
