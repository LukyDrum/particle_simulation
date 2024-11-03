use fastrand;

use crate::particles::constants::*;
use crate::particles::Particle;
use crate::Cell;
use crate::Neighborhood;
use crate::{Color, Offset};

use super::MatterType;
use super::ParticleChange;

const COLOR: u32 = 0xE0E02D;

#[derive(Clone)]
pub struct Sand {
    velocity: f32,
    color: Color,
    movement: Offset,
}

impl Sand {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: Color::hex(COLOR).similiar(),
            movement: Offset::new(0, 1),
        })
    }
}

impl Particle for Sand {
    fn get_name(&self) -> &str {
        "Sand"
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
        self.velocity
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_sand = self.clone();

        // Find new movement
        let rand_x = if fastrand::bool() { 1 } else { -1 };
        for_else!(
            for off in [Offset::new(0, 1), Offset::new(-rand_x, 1), Offset::new(rand_x, 1)] => {
                if let Some(cell) = neigborhood.on_relative(&off) {
                    match cell.get_particle() {
                        None => {
                            new_sand.movement = off;
                            // Check if the movement is down and apply gravity
                            if off.is_down() {
                                new_sand.velocity = MAX_VELOCITY.min(new_sand.velocity + GRAVITY);
                            }

                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_sand.movement = off;
                                // Apply some slowdown as if by friction of switching
                                new_sand.velocity = DEFAULT_VELOCITY.max(new_sand.velocity - SWITCH_SLOWDOWN);
                                break;
                            }
                        }
                    }
                }
            } else {
                new_sand.movement = Offset::zero();
                new_sand.velocity = DEFAULT_VELOCITY;
            }
        );

        ParticleChange::Changed(Some(Box::new(new_sand)))
    }
}
