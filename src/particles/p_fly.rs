use std::collections::LinkedList;

use rand::seq::SliceRandom;
use rand::{random, thread_rng};

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::{Burnability, ParticleChange};

const COLOR: u32 = 0xFF152E02;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u32 = 500;
const LIFETIME_OFF: u32 = 100;
const BURNABILITY_TIME: u8 = 10;

#[derive(Clone)]
pub struct Fly {
    color: u32,
    offsets: [Offset; 4],
    lifetime: u32,
    burnability: Burnability,
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
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
            burnability: Burnability::CanBurn,
        })
    }
}

impl Particle for Fly {
    fn get_name(&self) -> &str {
        "Fly"
    }

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

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn update(&self, neigborhood: super::Neighborhood) -> ParticleChange {
        // Lifetime reached 0 => fly is dead
        if self.lifetime == 0 {
            return ParticleChange::Changed(None);
        }

        // Clone fly and decrease it's lifetime by 1
        let mut new_fly = self.clone();
        new_fly.lifetime -= 1;

        // If the particle is burning => Destroy if time reached 0 else decrease the time by 1
        if let Burnability::IsBurning(time) = self.burnability {
            if time == 0 {
                return ParticleChange::Changed(None);
            } else {
                new_fly.burnability = self.burnability.decreased_by(1);
                return ParticleChange::Changed(Some(Box::new(new_fly)));
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

                    // Change color to FIRE_COLOR
                    new_fly.color = get_near_color(FIRE_COLOR);
                    new_fly.burnability = Burnability::IsBurning(BURNABILITY_TIME);
                    break;
                }
            }
        }

        ParticleChange::Changed(Some(Box::new(new_fly)))
    }
}
