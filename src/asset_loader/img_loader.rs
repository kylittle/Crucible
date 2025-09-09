use std::{fs::File, io::BufReader};

use dashmap::DashMap;
use image::ImageFormat;

use crate::utils::Color;

#[derive(Debug, Clone)]
pub struct RTWImage {
    colors: DashMap<(usize, usize), Color>,
    image_width: usize,
    image_height: usize,
}

impl RTWImage {
    /// Loads image data from a file in the folder assets
    pub fn new(image_filename: &str) -> RTWImage {
        // Get path to env folder or check a few directories above TODO: should probably do
        // this for all asset loaders so the assets folder can be found
        let image_filename =
            super::build_asset_path(image_filename).expect("Could not find the asset");

        // Now build the type based on the extension, and load in the image:
        let format = ImageFormat::from_path(&image_filename).expect("Unsupported filetype");
        let reader = BufReader::new(File::open(image_filename).unwrap());

        let image = image::load(reader, format).expect("Cannot read image");
        let image = image.to_rgb8();

        // Loop over the image and populate the dashmap
        let image_height = image.height();
        let image_width = image.width();

        let colors = DashMap::with_capacity((image_height * image_width) as usize);

        for h in 0..image_height {
            for w in 0..image_width {
                let pixel = image.get_pixel(w, h);

                let r = pixel.0[0] as f64 / 255.0;
                let g = pixel.0[1] as f64 / 255.0;
                let b = pixel.0[2] as f64 / 255.0;

                colors.insert((w as usize, h as usize), Color::new(r, g, b));
            }
        }

        drop(image.to_owned());

        RTWImage {
            colors,
            image_width: image_width as usize,
            image_height: image_height as usize,
        }
    }

    /// Gets the RTW images width
    pub fn width(&self) -> usize {
        self.image_width
    }

    /// Gets the RTW images height
    pub fn height(&self) -> usize {
        self.image_height
    }

    /// Returns the color at an x, y coordinate for the asset. If you are using this
    /// to place a texture you must convert the uv coordinates to x, y coordinates.
    pub fn pixel_data(&self, x: usize, y: usize) -> Color {
        // Should this be how the library works? It seems weird to fix the pixel coords
        // Maybe it should return none if its out of bounds?
        let x = x.clamp(0, self.image_width - 1);
        let y = y.clamp(0, self.image_height - 1);

        self.colors.get(&(x, y)).unwrap().value().clone()
    }
}
