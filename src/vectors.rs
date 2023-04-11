use std::f64::consts::PI;
 
pub type V2d = (i32, i32);

const ZERO: V2d = (0, 0);

pub trait Vector {
    fn x(&self) -> i32;

    fn y(&self) -> i32;

    fn dist(&self, other: Self) -> f64;

    fn abs(&self) -> f64;

    fn dot(&self, other: Self) -> i32;

    fn ang(&self, other: Self) -> i32;

    fn add(&self, other: Self) -> Self;

    fn scale(&self, s: i32) -> Self;
}

impl Vector for V2d {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn dist(&self, other: Self) -> f64 {
        (((self.0-other.0).pow(2) + (self.1-other.1).pow(2)) as f64).sqrt() as f64
    }

    fn abs(&self) -> f64 {
        self.dist(ZERO)
    }

    fn dot(&self, other: Self) -> i32 {
        self.0*other.0 + self.1*other.1
    }

    fn ang(&self, other: Self) -> i32 {
        let ndot = self.dot(other) as f64 / (self.abs() * other.abs());
        to_deg(ndot.acos())
    }

    fn add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn scale(&self, s: i32) -> Self {
        (self.0 * s, self.1 * s)
    }
}

// Theta is given in degrees.
// Note the resulting vector is rounded to the nearest integer.
pub fn from_polar(r: i32, theta: i32) -> V2d {
    let x = to_rad(theta).cos() * r as f64;
    let y = to_rad(theta).sin() * r as f64;
    (x as i32, y as i32)
}

fn to_deg(i: f64) -> i32 {
    (i * (180.0 / PI)) as i32
}

fn to_rad(i: i32) -> f64 {
    i as f64 * PI / 180.0
}
