use crate::vectors::{Vector, V2d, to_rad};

use super::{MAP_WIDTH, MAP_HEIGHT};

#[allow(dead_code)]
pub enum Ray {
    V(V2d, f64),
    H(V2d, f64),
}

impl Ray {
    pub fn to_vec(&self) -> V2d {
        match self {
            Ray::V(v, _) => *v,
            Ray::H(v, _) => *v,
        }
    }

    #[allow(dead_code)]
    pub fn to_dist(&self) -> f64 {
        match self {
            Ray::V(_, d) => *d,
            Ray::H(_, d) => *d,
        }
    }
}

#[derive(Debug)]
pub struct Keys {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

#[derive(Debug)]
pub struct Player {
    pub keys: Keys,
    pos: V2d,
    rot: f64,
    map: [u8;64]
}

impl Player {
    pub fn new(map: [u8;64]) -> Player {
        Player {
            pos: ((MAP_WIDTH / 2) as f64, (MAP_HEIGHT / 2) as f64),
            rot: 0.0,
            keys: Keys { w: false, a: false, s: false, d: false },
            map
        }
    }

    pub fn update(&mut self) {
        if self.keys.w {
            self.step(5.0);
        }
        if self.keys.s {
            self.step(-5.0);
        }
        if self.keys.a {
            self.rotate(-5.0);
        }
        if self.keys.d {
            self.rotate(5.0);
        }
    }

    pub fn pos(&self) -> V2d {
        self.pos
    }

    pub fn ray_cast(&self, offs: f64, map: &[u8]) -> Ray {
        let v_hit = self.ray_cast_v(offs, map);
        let h_hit = self.ray_cast_h(offs, map);
        if self.pos().dist(v_hit.to_vec()) < self.pos().dist(h_hit.to_vec()) {
            v_hit
        } else {
            h_hit
        }
    }

    fn ray_cast_v(&self, offs: f64, map: &[u8]) -> Ray {
        let mut dx: f64 = 10000.0;
        let mut dy: f64 = 10000.0;

        let mut rot = self.rot + offs;
        while rot < 0.0 {
           rot += 360.0 ;
        }
        while rot > 360.0 {
            rot -= 360.0
        }

        if rot < 90.0 || rot > 270.0 {
            let theta_tan = to_rad(rot).tan();
            dx = 64.0 - self.pos.0.rem_euclid(64.0);
            dy = theta_tan * dx;

            let y_ofs = theta_tan * 64.0;
            while !is_wall(self.pos.add((dx, dy)).add((1.0, 0.0)), map) {
                dx += 64.0;
                dy += y_ofs;
            }
        }

        if rot > 90.0 && rot < 270.0 {
            let theta_tan = to_rad(180.0 - rot).tan();
            dx = -self.pos.0.rem_euclid(64.0);
            dy = theta_tan * -dx;

            let y_ofs = theta_tan * 64.0;
            while !is_wall(self.pos.add((dx, dy)).add((-1.0, 0.0)), map) {
                dx -= 64.0;
                dy += y_ofs;
            }
        }

        let v = self.pos.add((dx, dy));
        let d = self.pos().dist(v) * to_rad(offs.abs()).cos();
        return Ray::V(v, d);
    }

    fn ray_cast_h(&self, offs: f64, map: &[u8]) -> Ray {
        let mut dx: f64 = 10000.0;
        let mut dy: f64 = 10000.0;

        let mut rot = self.rot + offs;
        while rot < 0.0 {
           rot += 360.0 ;
        }
        while rot > 360.0 {
            rot -= 360.0
        }

        if rot < 180.0 {
            let theta_tan = to_rad(90.0 - rot).tan();
            dy = 64.0 - self.pos.1.rem_euclid(64.0);
            dx = theta_tan * dy;

            let x_ofs = theta_tan * 64.0;
            while !is_wall(self.pos.add((dx, dy)).add((0.0, 1.0)), map) {
                dx += x_ofs;
                dy += 64.0;
            }
        }

        if rot > 180.0 {
            let theta_tan = to_rad(270.0 - rot).tan();
            dy = -self.pos.1.rem_euclid(64.0);
            dx = theta_tan * dy;

            let x_ofs = theta_tan * 64.0;
            while !is_wall(self.pos.add((dx, dy)).add((0.0, -1.0)), map) {
                dx -= x_ofs;
                dy -= 64.0;
            }
        }

        let v = self.pos.add((dx, dy));
        let d = self.pos().dist(v) * to_rad(offs.abs()).cos();
        return Ray::H(v, d);
    }

    fn step(&mut self, i: f64) {
        let dx = to_rad(self.rot).cos() * i;
        let dy = to_rad(self.rot).sin() * i;

        let cush = 15.0 * dx.signum();
        if !is_wall(self.pos.add((dx + cush, 0.0)), &self.map) {
            self.pos = self.pos.add((dx, 0.0));
        }

        let cush = 15.0 * dy.signum();
        if !is_wall(self.pos.add((0.0, dy + cush)), &self.map) {
            self.pos = self.pos.add((0.0, dy));
        }
    }

    fn rotate(&mut self, i: f64) {
        self.rot = self.rot + i;
        while self.rot > 360.0 {
            self.rot -= 360.0
        }
        while self.rot < 0.0 {
           self.rot += 360.0 
        }
    }
}

fn is_wall(v: V2d, map: &[u8]) -> bool {
    let (idx, idy) = (v.xi() / 64, v.yi() / 64);
    let id = 8 * idy + idx;
    if id < 0 || id >= 64 || map[id as usize] == 1 {
        true
    } else {
        false
    }
}

