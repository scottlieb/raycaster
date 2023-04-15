use crate::vectors::{Vector, V2d, to_rad};

use super::{MAP_WIDTH, MAP_HEIGHT};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical
}

// Hit represents an intersection of a cast ray with a wall.
pub struct Hit {
    // Orientation of the intersecting wall (horizontal or vertical).
    orn: Orientation,

    // Point of intersection.
    vec: V2d,

    // Ray angle offs relative to the player looking direction (in radians).
    offs_rad: f64,

    // Ray angle rotation relative to the player (in degrees).
    rot: f64
}

impl Hit {
    // Returns the point of intersection of the ray hit.
    pub fn to_vec(&self) -> V2d {
        self.vec
    }

    // Returns the orthonormal distance from the ray hit to the player.
    pub fn to_dist(&self, player: &Player) -> f64 {
        player.pos().dist(self.vec) * to_rad(self.offs_rad.abs()).cos()
    }

    // Returns the orientation of the intersecting wall.
    #[allow(dead_code)]
    pub fn to_orientation(&self) -> Orientation {
        self.orn
    }

    // Returns the relative offset of the hit within the block (for texture rendering).
    pub fn to_texture_offset(&self) -> u8 {
        match self.orn {
            Orientation::Vertical => {
                let mut t_offs = 64 - (self.vec.yi() % 64) as u8;
                if self.rot < 90.0 || self.rot > 270.0 {
                    t_offs = 64 - t_offs;
                }
                t_offs / 2
            },
            Orientation::Horizontal => {
                let mut t_offs = (self.vec.xi() % 64) as u8;
                if self.rot < 180.0 {
                    t_offs = 64 - t_offs;
                }
                t_offs / 2
            },
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

    pub fn ray_cast(&self, offs: f64, map: &[u8]) -> Hit {
        let v_hit = self.ray_cast_v(offs, map);
        let h_hit = self.ray_cast_h(offs, map);

        // Note: we take the shortest vector base on euclidian distance.
        // (As opposed to the orthonormal distance returned by the ray)
        if self.pos().dist(v_hit.to_vec()) < self.pos().dist(h_hit.to_vec()) {
            v_hit
        } else {
            h_hit
        }
    }

    fn ray_cast_v(&self, offs: f64, map: &[u8]) -> Hit {
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
        Hit { orn: Orientation::Vertical, vec: v, rot, offs_rad: offs }
    }

    fn ray_cast_h(&self, offs: f64, map: &[u8]) -> Hit {
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
        Hit { orn: Orientation::Horizontal, vec: v, rot, offs_rad: offs }
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

