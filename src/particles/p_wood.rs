use crate::particles::constants::*;
use crate::particles::Particle;
use crate::{Color, Offset};

use super::properties::PropertyCheckResult;
use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0x3D1812;
const DENSITY: u8 = MAX_DENSITY;
const BURNABILITY_TIME: u8 = 150;

#[derive(Clone)]
pub struct Wood {
    color: Color,
    burnability: Burnability,
}

impl Wood {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Wood {
            color: Color::hex(COLOR).similiar(),
            burnability: Burnability::CanBurn,
        })
    }
}

impl Particle for Wood {
    fn get_name(&self) -> &str {
        "Wood"
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn is_moveable(&self) -> bool {
        false
    }

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn set_burnability(&mut self, new_burnability: Burnability) -> () {
        self.burnability = new_burnability;
    }

    fn get_movement(&self) -> Offset {
        Offset::zero()
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_wood = self.clone();

        let res = Burnability::check(&mut new_wood, &neigborhood, BURNABILITY_TIME, true);

        match res {
            PropertyCheckResult::Updated => {
                if let Burnability::IsBurning(_) = new_wood.get_burnability() {
                    new_wood.color = Color::hex(FIRE_COLOR).similiar();
                }

                ParticleChange::Changed(Some(Box::new(new_wood)))
            }
            PropertyCheckResult::Destroyed => ParticleChange::Changed(None),
            PropertyCheckResult::None => ParticleChange::None,
        }
    }
}
