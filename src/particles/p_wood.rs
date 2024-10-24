use std::collections::LinkedList;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::properties::PropertyCheckResult;
use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0xFF3D1812;
const DENSITY: u8 = MAX_DENSITY;
const BURNABILITY_TIME: u8 = 150;

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

    fn _get_offsets(&self) -> LinkedList<Offset> {
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

    fn set_burnability(&mut self, new_burnability: Burnability) -> () {
        self.burnability = new_burnability;
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_wood = self.clone();

        let res = Burnability::check(&mut new_wood, &neigborhood, BURNABILITY_TIME, true);

        match res {
            PropertyCheckResult::Updated => {
                if let Burnability::IsBurning(_) = new_wood.get_burnability() {
                    new_wood.color = get_near_color(FIRE_COLOR);
                }

                ParticleChange::Changed(Some(Box::new(new_wood)))
            }
            PropertyCheckResult::Destroyed => ParticleChange::Changed(None),
            PropertyCheckResult::None => ParticleChange::None,
        }
    }
}
