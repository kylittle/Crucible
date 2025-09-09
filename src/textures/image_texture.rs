use crate::{
    asset_loader::img_loader::RTWImage,
    textures::Texture,
    utils::{Color, Interval, Point3},
};

/// A Texture with an underlying image. See asset_loader for
/// details of how an image can be loaded
#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let image = RTWImage::new(filename);

        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let image_interval = Interval::new(0.0, 1.0);
        let u = image_interval.clamp(u);
        let v = 1.0 - image_interval.clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;

        self.image.pixel_data(i, j)
    }
}
