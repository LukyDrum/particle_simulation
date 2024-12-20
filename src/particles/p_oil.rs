use crate::particles::constants::*;
use crate::particles::Particle;
use crate::Neighborhood;
use crate::{Color, Offset};

use super::properties::PropertyCheckResult;
use super::MatterType;
use super::{Burnability, ParticleChange, Smoke};

const COLOR: u32 = 0x996E17;
const DENSITY: u8 = 120;
const BURNABILITY_TIME: u8 = 100;

#[derive(Clone)]
pub struct Oil {
    velocity: f32,
    color: Color,
    burnability: Burnability,
    movement: Offset,
    x_dir: i32,
}

impl Oil {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Oil {
            velocity: DEFAULT_VELOCITY,
            color: Color::hex(COLOR).similiar(),
            burnability: Burnability::CanBurn,
            movement: Offset::zero(),
            x_dir: if fastrand::bool() { 1 } else { -1 },
        })
    }
}

impl Particle for Oil {
    fn get_name(&self) -> &str {
        "Oil"
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_matter_type(&self) -> &MatterType {
        &MatterType::Liquid
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn get_velocity(&self) -> f32 {
        self.velocity
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

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_oil = self.clone();

        // Check in direction of x_dir for obstacels or out of bounds and move away from them
        let in_x_dir = neigborhood.on_relative(&Offset::new(new_oil.x_dir, 0));
        if let Some(cell) = in_x_dir {
            if let Some(_) = cell.get_particle() {
                new_oil.x_dir = -new_oil.x_dir;
            }
        } else {
            new_oil.x_dir = -new_oil.x_dir;
        }

        // Find new movement
        for_else!(
            for off in [Offset::new(0, 1), Offset::new(new_oil.x_dir, 0), Offset::new(-new_oil.x_dir, 0)] => {
                if let Some(cell) = neigborhood.on_relative(&off) {
                    match cell.get_particle() {
                        None => {
                            new_oil.movement = off;
                            // Check if the movement is down and apply gravity
                            if off.is_down() {
                                new_oil.velocity = MAX_VELOCITY.min(new_oil.velocity + GRAVITY);
                            }
                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_oil.movement = off;
                                // Apply some slowdown as if by friction of switching
                                new_oil.velocity = DEFAULT_VELOCITY.max(new_oil.velocity - SWITCH_SLOWDOWN);
                                break;
                            }
                        }
                    }
                }
            } else {
                new_oil.movement = Offset::zero();
                new_oil.velocity = DEFAULT_VELOCITY;
            }
        );

        let res = Burnability::check(&mut new_oil, &neigborhood, BURNABILITY_TIME, true);
        match res {
            PropertyCheckResult::Updated => {
                if let Burnability::IsBurning(_) = new_oil.get_burnability() {
                    new_oil.color = Color::hex(FIRE_COLOR).similiar();
                }

                ParticleChange::Changed(Some(Box::new(new_oil)))
            }
            PropertyCheckResult::Destroyed => ParticleChange::Changed(Some(Smoke::new())),
            PropertyCheckResult::None => ParticleChange::Changed(Some(Box::new(new_oil))),
        }
    }
}
