use fastrand;

use crate::particles::{constants::*, NeighborCell};
use crate::particles::{get_near_color, Particle};
use crate::Offset;

use super::{Neighborhood, ParticleChange};

const COLOR: u32 = 0xFFE0E02D;

#[derive(Clone)]
pub struct Sand {
    velocity: f32,
    color: u32,
    movement: Offset,
}

impl Sand {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            movement: Offset::new(0, 1),
        })
    }

    pub fn with_color(color: u32) -> Box<dyn Particle> {
        Box::new(Sand {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(color),
            movement: Offset::new(0, 1),
        })
    }
}

impl Particle for Sand {
    fn get_name(&self) -> &str {
        "Sand"
    }

    fn get_color(&self) -> u32 {
        self.color
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

    fn is_solid(&self) -> bool {
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
                if let NeighborCell::Inside(opt) = neigborhood.on_relative(&off) {
                    match opt {
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
