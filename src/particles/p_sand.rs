use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::{Neighborhood, ParticleChange};

const COLOR: u32 = 0xFFE0E02D;

#[derive(Clone)]
pub struct Sand {
    velocity: f32,
    color: u32,
    movement: Offset,
}

impl Sand {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            movement: Offset::new(0, 1),
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
            movement: Offset::new(0, 1),
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

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();
        lst.push_back(self.movement * (self.velocity as i32));
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

    fn _get_offsets(&self) -> LinkedList<Offset> {
        LinkedList::new()
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_sand = self.clone();

        for_else!(
            for off in [Offset::new(0, 1), Offset::new(-1, 1), Offset::new(1, 1)] => {
                match neigborhood.on_relative(&off) {
                    None => {
                        new_sand.movement = off;
                        break;
                    }
                    Some(other) => {
                        if self.can_switch_with(other) {
                            new_sand.movement = off;
                            break;
                        }
                    }
                }
            } else {
                new_sand.movement = Offset::zero();
                new_sand.velocity = DEFAULT_VELOCITY;
            }
        );

        ParticleChange::Changed(Some(Box::new(new_sand)))
    }
}
