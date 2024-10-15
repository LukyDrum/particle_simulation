use std::collections::LinkedList;

use crate::particles::particle::{DEFAULT_VELOCITY, GRAVITY, MAX_DENSITY, MAX_VELOCITY};
use crate::particles::Particle;
use crate::Offset;

const SAND_COLOR: u32 = 0x00E0E02D;

pub struct Sand {
    velocity: f32,
}

impl Sand {
    pub fn new() -> Sand {
        Sand {
            velocity: DEFAULT_VELOCITY,
        }
    }
}

impl Particle for Sand {
    fn get_color(&self) -> u32 {
        SAND_COLOR
    }

    fn get_density(&self) -> u8 {
        MAX_DENSITY
    }

    fn get_velocity(&self) -> f32 {
        self.velocity
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();

        lst.push_back(Offset::new(0, 1));
        lst.push_back(Offset::new(1, 0));
        lst.push_back(Offset::new(-1, 0));

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        true
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_gravity(&mut self) -> () {
        self.velocity = (self.velocity + GRAVITY).min(MAX_VELOCITY);
    }
}
