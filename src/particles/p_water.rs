use fastrand;

use crate::particles::{constants::*, NeighborCell, Vapor};
use crate::particles::{get_near_color, Particle};
use crate::Offset;

// use super::{Burnability, Neighborhood, ParticleChange, Vapor};
use super::{Burnability, Neighborhood, ParticleChange};

const COLOR: u32 = 0xFF326ECF;
const DENSITY: u8 = 128;

#[derive(Clone)]
pub struct Water {
    velocity: f32,
    color: u32,
    movement: Offset,
    x_dir: i32, // Used to keep water keeping in one side direction until it can no longer - helps with spreading
}

impl Water {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Water {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            movement: Offset::new(0, 1),
            x_dir: if fastrand::bool() { 1 } else { -1 }, // Start with a random x_dir
        })
    }
}

impl Particle for Water {
    fn get_name(&self) -> &str {
        "Water"
    }

    fn get_color(&self) -> u32 {
        self.color
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

    fn is_solid(&self) -> bool {
        false
    }

    fn get_burnability(&self) -> Burnability {
        Burnability::AntiBurn
    }

    fn get_movement(&self) -> Offset {
        self.movement * self.velocity as i32
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        let mut new_water = self.clone();

        let left = neigborhood.left();
        let right = neigborhood.right();
        // Check left and right for new x_dir and move away from obstacles
        if left.is_some() || left.is_outside() {
            new_water.x_dir = 1;
        } else if right.is_some() || right.is_outside() {
            new_water.x_dir = -1;
        }

        // Find new movement
        for_else!(
            for off in [Offset::new(0, 1), Offset::new(new_water.x_dir, 0), Offset::new(-new_water.x_dir, 0)] => {
                if let NeighborCell::Inside(opt) = neigborhood.on_relative(&off) {
                    match opt {
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
            if let NeighborCell::Inside(Some(neigh)) = opt {
                match neigh.get_burnability() {
                    Burnability::IsBurning(_) => count += 1,
                    Burnability::AntiBurn => count -= 1,
                    _ => {}
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
