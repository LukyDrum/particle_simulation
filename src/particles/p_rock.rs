use std::collections::LinkedList;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

const COLOR: u32 = 0x00474747;
const DENSITY: u8 = MAX_DENSITY;

#[derive(Clone)]
pub struct Rock {
    velocity: f32,
    color: u32,
}

impl Rock {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Rock {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
        })
    }
}

impl Particle for Rock {
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
        LinkedList::new()
    }

    fn is_moveable(&self) -> bool {
        false
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
