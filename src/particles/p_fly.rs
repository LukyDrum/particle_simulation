use std::collections::LinkedList;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

const COLOR: u32 = 0xFF152E02;

#[derive(Clone)]
pub struct Fly {
    color: u32,
    offsets: [Offset; 4],
}

impl Fly {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Fly {
            color: get_near_color(COLOR),
            offsets: [
                Offset::new(1, 0),
                Offset::new(-1, 0),
                Offset::new(0, 1),
                Offset::new(0, -1),
            ],
        })
    }
}

impl Particle for Fly {
    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        MAX_DENSITY
    }

    fn get_velocity(&self) -> f32 {
        DEFAULT_VELOCITY
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut indexes: Vec<usize> = (0..self.offsets.len()).collect();
        indexes.shuffle(&mut thread_rng());

        let mut lst = LinkedList::new();
        for i in indexes {
            lst.push_back(self.offsets[i]);
        }

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        true
    }

    fn reset_velocity(&mut self) -> () {}

    fn apply_acceleration(&mut self, _acc: f32) -> () {}
}