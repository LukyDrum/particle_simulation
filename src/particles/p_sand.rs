use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

const COLOR: u32 = 0xFFE0E02D;

#[derive(Clone)]
pub struct Sand {
    velocity: f32,
    color: u32,
}

impl Sand {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
        })
    }
}

impl Particle for Sand {
    fn get_name(&self) -> &str {
        "Sand"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        MAX_DENSITY
    }

    fn get_velocity(&self) -> f32 {
        self.velocity
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();
        let vel = self.velocity as i32;

        lst.push_back(Offset::new(0, 1) * vel);
        if random() {
            lst.push_back(Offset::new(1, 1) * vel);
            lst.push_back(Offset::new(-1, 1) * vel);
        } else {
            lst.push_back(Offset::new(-1, 1) * vel);
            lst.push_back(Offset::new(1, 1) * vel);
        }

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

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }
}
