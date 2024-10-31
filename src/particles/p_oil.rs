use crate::particles::Particle;
use crate::particles::{constants::*, NeighborCell};
use crate::{Color, Offset};

use super::properties::PropertyCheckResult;
use super::{Burnability, Neighborhood, ParticleChange, Smoke};

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

        let left = neigborhood.left();
        let right = neigborhood.right();
        // Check left and right for new x_dir and move away from obstacles
        if left.is_some() && right.is_some() {
            // Might be possible to switch
            new_oil.x_dir = if fastrand::bool() { 1 } else { -1 };
        } else if left.is_some() || left.is_outside() {
            new_oil.x_dir = 1;
        } else if right.is_some() || right.is_outside() {
            new_oil.x_dir = -1;
        }

        // Find new movement
        for_else!(
            for off in [Offset::new(0, 1), Offset::new(new_oil.x_dir, 0), Offset::new(-new_oil.x_dir, 0)] => {
                if let NeighborCell::Inside(opt) = neigborhood.on_relative(&off) {
                    match opt {
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
