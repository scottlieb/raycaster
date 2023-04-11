use std::f64::consts::PI;
 
pub type V2d = (f64, f64);

const ZERO: V2d = (0.0, 0.0);

pub trait Vector {
    fn xi(&self) -> i32;

    fn yi(&self) -> i32;

    fn dist(&self, other: Self) -> f64;

    fn abs(&self) -> f64;

    fn dot(&self, other: Self) -> f64;

    fn ang(&self, other: Self) -> f64;

    fn add(&self, other: Self) -> Self;

    fn scale(&self, s: f64) -> Self;
}

impl Vector for V2d {
    fn xi(&self) -> i32 {
        self.0 as i32
    }

    fn yi(&self) -> i32 {
        self.1 as i32
    }

    fn dist(&self, other: Self) -> f64 {
        (((self.0-other.0).powf(2.0) + (self.1-other.1).powf(2.0))).sqrt()
    }

    fn abs(&self) -> f64 {
        self.dist(ZERO)
    }

    fn dot(&self, other: Self) -> f64 {
        self.0*other.0 + self.1*other.1
    }

    fn ang(&self, other: Self) -> f64 {
        let ndot = self.dot(other) / (self.abs() * other.abs());
        to_deg(ndot.acos())
    }

    fn add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn scale(&self, s: f64) -> Self {
        (self.0 * s, self.1 * s)
    }
}

// Theta is given in degrees.
// Note the resulting vector is rounded to the nearest integer.
pub fn from_polar(r: f64, theta: f64) -> V2d {
    let x = to_rad(theta).cos() * r;
    let y = to_rad(theta).sin() * r;
    (x, y)
}

fn to_deg(i: f64) -> f64 {
    i * (180.0 / PI)
}

pub fn to_rad(i: f64) -> f64 {
    i * PI / 180.0
}
