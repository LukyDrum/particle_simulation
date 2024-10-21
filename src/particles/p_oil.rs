use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0xFF996E17;
const DENSITY: u8 = 120;
const BURNABILITY_TIME: u8 = 32;

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

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        // If the particle is burning => Destroy if time reached 0 else decrease the time by 1
        if let Burnability::IsBurning(time) = self.burnability {
            if time == 0 {
                return ParticleChange::Changed(None);
            } else {
                let mut new_p = self.clone();
                new_p.burnability = self.burnability.decreased_by(1);
                return ParticleChange::Changed(Some(Box::new(new_p)));
            }
        }

        // Check neighbors, if any one of them is burning => set this particle as burning with default time.
        for opt in neigborhood.iter().flatten() {
            if let Some(neigh) = opt {
                if let Burnability::IsBurning(_) = neigh.get_burnability() {
                    // Chance not to catch fire
                    if random() {
                        continue;
                    }

                    let mut new_p = self.clone();
                    // Change color to FIRE_COLOR
                    new_p.color = get_near_color(FIRE_COLOR);
                    new_p.burnability = Burnability::IsBurning(BURNABILITY_TIME);
                    return ParticleChange::Changed(Some(Box::new(new_p)));
                }
            }
        }

        // None of the above met => no change
        ParticleChange::None
    }
}
