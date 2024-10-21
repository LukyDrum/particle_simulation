use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0xFF3D1812;
const DENSITY: u8 = MAX_DENSITY;
const BURNABILITY_TIME: u8 = 60;

#[derive(Clone)]
pub struct Wood {
    color: u32,
    burnability: Burnability,
}

impl Wood {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Wood {
            color: get_near_color(COLOR),
            burnability: Burnability::CanBurn,
        })
    }
}

impl Particle for Wood {
    fn get_name(&self) -> &str {
        "Wood"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
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

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        // If the particle is burning => Destroy if time reached 0 else decrease the time by 1
        if let Burnability::IsBurning(time) = self.burnability {
            if time == 0 {
                return ParticleChange::Changed(None);
            } else {
                let mut new_p = self.clone();
                new_p.color = get_near_color(FIRE_COLOR); // Make the color change a little
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
