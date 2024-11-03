use fastrand;

use crate::particles::Particle;
use crate::particles::{constants::*, Vapor};
use crate::{Cell, Color, Neighborhood, Offset};

// use super::{Burnability, Neighborhood, ParticleChange, Vapor};
use super::{Burnability, MatterType, ParticleChange};

const COLOR: u32 = 0x326ECF;
const DENSITY: u8 = 128;

#[derive(Clone)]
pub struct Water {
    velocity: f32,
    color: Color,
    movement: Offset,
    x_dir: i32, // Used to keep water keeping in one side direction until it can no longer - helps with spreading
}

impl Water {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: Color::hex(COLOR).similiar(),
            movement: Offset::new(0, 1),
            x_dir: if fastrand::bool() { 1 } else { -1 }, // Start with a random x_dir
        })
    }
}

impl Particle for Water {
    fn get_name(&self) -> &str {
        "Water"
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
        Burnability::AntiBurn
    }

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_water = self.clone();

        // Check in direction of x_dir for obstacels or out of bounds and move away from them
        let in_x_dir = neigborhood.on_relative(&Offset::new(new_water.x_dir, 0));
        if let Some(cell) = in_x_dir {
            if let Some(_) = cell.get_particle() {
                new_water.x_dir = -new_water.x_dir;
            }
        } else {
            new_water.x_dir = -new_water.x_dir;
        }

        // Find new movement
        for_else!(
            for off in [Offset::new(0, 1), Offset::new(new_water.x_dir, 0), Offset::new(-new_water.x_dir, 0)] => {
                if let Some(cell) = neigborhood.on_relative(&off) {
                    match cell.get_particle() {
                        None => {
                            new_water.movement = off;
                            // Check if the movement is down and apply gravity
                            if off.is_down() {
                                new_water.velocity = MAX_VELOCITY.min(new_water.velocity + GRAVITY);
                            }
                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_water.movement = off;
                                // Apply some slowdown as if by friction of switching
                                new_water.velocity = DEFAULT_VELOCITY.max(new_water.velocity - SWITCH_SLOWDOWN);
                                break;
                            }
                        }
                    }
                }
            } else {
                new_water.movement = Offset::zero();
                new_water.velocity = DEFAULT_VELOCITY;
            }
        );

        // Check number of neighbors that are IsBurning and AntiBurn
        let mut count = 0;
        for opt in neigborhood.iter() {
            if let Some(cell) = opt {
                if let Some(neigh) = cell.get_particle() {
                    match neigh.get_burnability() {
                        Burnability::IsBurning(_) => count += 1,
                        Burnability::AntiBurn => count -= 1,
                        _ => {}
                    }
                }
            }
        }

        if count > 0 {
            ParticleChange::Changed(Some(Vapor::new()))
        } else {
            ParticleChange::Changed(Some(Box::new(new_water)))
        }
    }
}
