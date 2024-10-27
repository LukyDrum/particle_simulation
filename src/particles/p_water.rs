use std::collections::LinkedList;

use rand::{random, thread_rng};

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

// use super::{Burnability, Neighborhood, ParticleChange, Vapor};
use super::{Burnability, Neighborhood, ParticleChange, Sand};

const COLOR: u32 = 0xFF326ECF;
const DENSITY: u8 = 128;

#[derive(Clone)]
pub struct Water {
    velocity: f32,
    color: u32,
    x_dir: i32,
}

impl Water {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            x_dir: Self::random_x_dir(),
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
            x_dir: Self::random_x_dir(),
        })
    }

    fn random_x_dir() -> i32 {
        if random() {
            -1
        } else {
            1
        }
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
        let mut lst = LinkedList::new();
        let vel = self.velocity as i32;

        lst.push_back(Offset::new(0, 1) * vel);
        lst.push_back(Offset::new(self.x_dir, 1) * vel);
        lst.push_back(Offset::new(-self.x_dir, 1) * vel);
        lst.push_back(Offset::new(self.x_dir, 0) * vel);
        lst.push_back(Offset::new(-self.x_dir, 0) * vel);

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

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
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
            // Check left and right for direction change
            // Left
            if self.x_dir == -1 {
                if let Some(p) = &neigborhood.left() {
                    if p.get_density() >= self.get_density() {
                        let mut new_water = self.clone();
                        new_water.x_dir = 1;
                        return ParticleChange::Changed(Some(Box::new(new_water)));
                    }
                }
            } else if self.x_dir == 1 {
                if let Some(p) = &neigborhood.right() {
                    if p.get_density() >= self.get_density() {
                        let mut new_water = self.clone();
                        new_water.x_dir = -1;
                        return ParticleChange::Changed(Some(Box::new(new_water)));
                    }
                }
            }

            ParticleChange::None
        }
    }
}
