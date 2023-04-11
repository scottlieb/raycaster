use crate::vectors::{Vector, V2d, from_polar};

use super::{WIDTH, HEIGHT};
use std::cmp::{min, max};

// #[derive(Debug)]
// struct Keys {
//     w: u8,
//     a: u8,
//     s: u8,
//     d: u8,
// }

#[derive(Debug)]
pub struct Player {
    pos: V2d,
    rot: i32,
    // keys: Keys,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (WIDTH / 2, HEIGHT / 2),
            rot: 0,
            // keys: Keys { w: 0, a: 0, s: 0, d: 0 }
        }
    }

    pub fn step(&mut self, i: i32) {
        let mut new_pos = self.pos.add(from_polar(i, self.rot));

        // Check bounds. TODO: improve.
        new_pos.0 = max(new_pos.0, 5);
        new_pos.0 = min(new_pos.0, WIDTH - 5);
        new_pos.1 = max(new_pos.1, 5);
        new_pos.1 = min(new_pos.1, HEIGHT - 5);

        self.pos = new_pos;
    }

    pub fn rotate(&mut self, i: i32) {
        self.rot = (self.rot + i) % 360;
    }

    // Ray  returns a vector of length l representing
    // the rotation of the player.
    pub fn ray(&self, l: i32) -> V2d {
        from_polar(l, self.rot)
    }

    pub fn pos(&self) -> V2d {
        self.pos
    }
}
