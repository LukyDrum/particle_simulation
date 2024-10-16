use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

const COLOR: u32 = 0xFF996E17;
const DENSITY: u8 = 120;

#[derive(Clone)]
pub struct Oil {
    velocity: f32,
    color: u32,
}

impl Oil {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Oil {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
        })
    }
}

impl Particle for Oil {
    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn get_velocity(&self) -> f32 {
        self.velocity
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();
        let vel = self.velocity as i32;

        lst.push_back(Offset::new(0, 1) * vel);
        if random() {
            lst.push_back(Offset::new(1, 0) * vel);
            lst.push_back(Offset::new(-1, 0) * vel);
        } else {
            lst.push_back(Offset::new(-1, 0) * vel);
            lst.push_back(Offset::new(1, 0) * vel);
        }

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }
}
