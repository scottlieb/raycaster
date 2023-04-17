use image::{DynamicImage, GenericImageView, Pixel};

pub const TEXTURE_SIZE: usize = 64;
type Colour = (u8, u8, u8);

pub fn get_texture() -> image::DynamicImage {
    image::open("resources/mossy.png").unwrap()
}

pub trait Tex {
    fn get_col(&self, x: u8, y: u8) -> Colour;
}

impl Tex for DynamicImage {
    fn get_col(&self, x: u8, y: u8) -> Colour {
        let p = self.get_pixel(x as u32, y as u32);
        let rgb = p.to_rgb();
        (rgb[0], rgb[1], rgb[2])
    }
}

pub trait Col {
    fn darken(&self, s: f64) -> Self;
}

impl Col for Colour {
    fn darken(&self, s: f64) -> Colour {
        if s < 0.0 || s >= 1.0 {
            return *self;
        }

        let r = (self.0 as f64 * s) as u8;
        let g = (self.1 as f64 * s) as u8;
        let b = (self.2 as f64 * s) as u8;

        (r, g, b)
    }
}
