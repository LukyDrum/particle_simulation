use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

// use super::{Burnability, Neighborhood, ParticleChange, Vapor};
use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0xFF326ECF;
const DENSITY: u8 = 128;

#[derive(Clone)]
pub struct Water {
    velocity: f32,
    color: u32,
    movement: Offset,
}

impl Water {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            movement: Offset::new(0, 1),
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
            movement: Offset::new(0, 1),
        })
    }
}

impl Particle for Water {
    fn get_name(&self) -> &str {
        "Water"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn get_velocity(&self) -> f32 {
        self.velocity
    }

    fn _get_offsets(&self) -> LinkedList<Offset> {
        LinkedList::new()
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
        false
    }

    fn get_burnability(&self) -> Burnability {
        Burnability::AntiBurn
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_water = self.clone();

        for_else!(
            for off in [Offset::new(0, 1), Offset::new(-1, 1), Offset::new(1, 1), Offset::new(-1, 0), Offset::new(1, 0)] => {
                match neigborhood.on_relative(&off) {
                    None => {
                        new_water.movement = off;
                        break;
                    }
                    Some(other) => {
                        if self.can_switch_with(other) {
                            new_water.movement = off;
                            break;
                        }
                    }
                }
            } else {
                new_water.movement = Offset::zero();
                new_water.velocity = DEFAULT_VELOCITY;
            }
        );

        // Check number of neighbors that are IsBurning and AntiBurn
        let mut count = 0;
        for opt in neigborhood.iter() {
            if let Some(neigh) = opt {
                match neigh.get_burnability() {
                    Burnability::IsBurning(_) => count += 1,
                    Burnability::AntiBurn => count -= 1,
                    _ => {}
                }
            }
        }

        if count > 0 {
            // ParticleChange::Changed(Some(Vapor::new()))
            ParticleChange::None
        } else {
            ParticleChange::Changed(Some(Box::new(new_water)))
        }
    }
}
