pub const TEXTURE_SIZE: usize = 32;
const B: Colour = (0x22, 0x22, 0x22);
const W: Colour = (0xee, 0xee, 0xee);

const TEXTURE_1: Texture = [
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
    W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, W, W, W, W, W, W, W, W, B, B, B, B, B, B, B, B, 
];

type Colour = (u8, u8, u8);
type Texture =  [Colour; TEXTURE_SIZE * TEXTURE_SIZE];

pub fn get_texture() -> Texture {
    TEXTURE_1
}

pub trait Tex {
    fn get_col(&self, x: u8, y: u8) -> Colour;
}

impl Tex for Texture {
    fn get_col(&self, x: u8, y: u8) -> Colour {
        let idx = y as usize * TEXTURE_SIZE + x as usize;
        if idx >= 1024 {
            return B;
        }
        self[idx]
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
