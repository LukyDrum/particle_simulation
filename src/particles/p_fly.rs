use crate::particles::constants::*;
use crate::particles::Particle;
use crate::utility::get_value_around;
use crate::Neighborhood;
use crate::{Color, Offset};
use fastrand;

use super::properties::PropertyCheckResult;
use super::MatterType;
use super::{Burnability, ParticleChange};

const COLOR: u32 = 0x152E02;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u32 = 1000;
const LIFETIME_OFF: u32 = 400;
const BURNABILITY_TIME: u8 = 25;
const FOCUS_TIME: u32 = 7; // How long can a fly focus on a single direction of movement
const FOCUS_TIME_OFFSET: u32 = 7;
const OFFSETS: [Offset; 9] = [
    Offset { x: 0, y: 0 },
    Offset { x: 1, y: 0 },
    Offset { x: -1, y: 0 },
    Offset { x: 0, y: 1 },
    Offset { x: 0, y: -1 },
    Offset { x: 1, y: 1 },
    Offset { x: -1, y: 1 },
    Offset { x: 1, y: -1 },
    Offset { x: -1, y: -1 },
];

#[derive(Clone)]
pub struct Fly {
    color: Color,
    lifetime: u32,
    burnability: Burnability,
    movement: Offset,
    focus: u32,
}

impl Fly {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Fly {
            color: Color::hex(COLOR).similiar(),
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
            burnability: Burnability::CanBurn,
            movement: Offset::zero(),
            focus: 0,
        })
    }
}

impl Particle for Fly {
    fn get_name(&self) -> &str {
        "Fly"
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_matter_type(&self) -> &MatterType {
        &MatterType::Solid
    }

    fn get_density(&self) -> u8 {
        MAX_DENSITY
    }

    fn get_velocity(&self) -> f32 {
        DEFAULT_VELOCITY
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn get_burnability(&self) -> Burnability {
        self.burnability
    }

    fn set_burnability(&mut self, new_burnability: Burnability) -> () {
        self.burnability = new_burnability;
    }

    fn can_switch_with(&self, other: &Box<dyn Particle>) -> bool {
        // Can switch but only if the other particle is gas (has the density of gas)
        other.get_density() < MAX_GAS_DENSITY
    }

    fn get_movement(&self) -> Offset {
        self.movement
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        // Lifetime reached 0 => fly is dead
        if self.lifetime == 0 {
            return ParticleChange::Changed(None);
        }

        // Clone fly and decrease it's lifetime by 1
        let mut new_fly = self.clone();
        new_fly.lifetime -= 1;

        // Find new movement
        let on_next_cell = neigborhood.on_relative(&self.movement);
        // !is_none = !(inside AND none) = outisde OR some
        if new_fly.focus == 0 || !on_next_cell.is_none() {
            let mut indexes: Vec<usize> = (0..OFFSETS.len()).collect();
            fastrand::shuffle(indexes.as_mut_slice());
            // Loop over offsets indexed by shuffled
            for_else!(
                for index in indexes => {
                    let off = OFFSETS[index];
                    if let Some(cell) = neigborhood.on_relative(&off) {
                        match cell.get_particle() {
                            None => {
                                new_fly.movement = off;
                                new_fly.focus = get_value_around(FOCUS_TIME, FOCUS_TIME_OFFSET);
                                break;
                            }
                            Some(other) => {
                                if new_fly.can_switch_with(other) {
                                    new_fly.movement = off;
                                    new_fly.focus = get_value_around(FOCUS_TIME, FOCUS_TIME_OFFSET);
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    new_fly.movement = Offset::zero();
                }
            );
        } else {
            new_fly.focus -= 1;
        }

        let res = Burnability::check(&mut new_fly, &neigborhood, BURNABILITY_TIME, true);
        match res {
            PropertyCheckResult::Updated => {
                if let Burnability::IsBurning(_) = new_fly.get_burnability() {
                    new_fly.color = Color::hex(FIRE_COLOR).similiar();
                }

                ParticleChange::Changed(Some(Box::new(new_fly)))
            }
            PropertyCheckResult::Destroyed => ParticleChange::Changed(None),
            PropertyCheckResult::None => ParticleChange::Changed(Some(Box::new(new_fly))),
        }
    }
}
