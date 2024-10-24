use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::properties::PropertyCheckResult;
use super::{Burnability, Neighborhood, ParticleChange, Smoke};

const COLOR: u32 = 0xFF996E17;
const DENSITY: u8 = 120;
const BURNABILITY_TIME: u8 = 100;

#[derive(Clone)]
pub struct Oil {
    velocity: f32,
    color: u32,
    burnability: Burnability,
}

impl Oil {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Oil {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            burnability: Burnability::CanBurn,
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Oil {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
            burnability: Burnability::CanBurn,
        })
    }
}

impl Particle for Oil {
    fn get_name(&self) -> &str {
        "Oil"
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

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn set_burnability(&mut self, new_burnability: Burnability) -> () {
        self.burnability = new_burnability;
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_oil = self.clone();

        let res = Burnability::check(&mut new_oil, &neigborhood, BURNABILITY_TIME, true);

        match res {
            PropertyCheckResult::Updated => {
                if let Burnability::IsBurning(_) = new_oil.get_burnability() {
                    new_oil.color = get_near_color(FIRE_COLOR);
                }

                ParticleChange::Changed(Some(Box::new(new_oil)))
            }
            PropertyCheckResult::Destroyed => ParticleChange::Changed(Some(Smoke::new())),
            PropertyCheckResult::None => ParticleChange::None,
        }
    }
}
