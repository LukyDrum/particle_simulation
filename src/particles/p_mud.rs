use rand::random;

use crate::particles::{constants::*, NeighborCell};
use crate::particles::{get_near_color, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::ParticleChange;

const COLOR: u32 = 0xFF91473D;
const DEFAULT_SIDE_FRICTION: u8 = 16;
const FRICTION_OFF: u8 = 4;

#[derive(Clone)]
pub struct Mud {
    velocity: f32,
    color: u32,
    /// Definies how much does the mud particle want to fall to the side. Actually falls when it reaches zero
    side_friction: u8,
    movement: Offset,
}

impl Mud {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Mud {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            side_friction: get_side_friction(),
            movement: Offset::zero(),
        })
    }
}

impl Particle for Mud {
    fn get_name(&self) -> &str {
        "Mud"
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

    fn update(&self, neigborhood: super::Neighborhood) -> ParticleChange {
        let mut new_mud = self.clone();

        // Empty cell bellow or full but can switch
        match neigborhood.down() {
            NeighborCell::Inside(None) => {
                new_mud.movement = Offset::new(0, 1);
                new_mud.velocity = MAX_VELOCITY.min(new_mud.velocity + GRAVITY);

                return ParticleChange::Changed(Some(Box::new(new_mud)));
            }
            NeighborCell::Inside(Some(other)) => {
                if new_mud.can_switch_with(other) {
                    new_mud.movement = Offset::new(0, 1);
                    // Apply some slowdown as if by friction of switching
                    new_mud.velocity = DEFAULT_VELOCITY.max(new_mud.velocity - SWITCH_SLOWDOWN);

                    return ParticleChange::Changed(Some(Box::new(new_mud)));
                }
            }
            _ => {}
        }

        // Cant fall to side yet
        if new_mud.side_friction > 0 {
            new_mud.movement = Offset::zero();
            new_mud.velocity = DEFAULT_VELOCITY;
            new_mud.side_friction -= 1;
            return ParticleChange::Changed(Some(Box::new(new_mud)));
        }

        // Find new movement to sides, because side friction is 0
        let rand_x = if fastrand::bool() { 1 } else { -1 };
        for_else!(
            for off in [Offset::new(-rand_x, 1), Offset::new(rand_x, 1)] => {
                if let NeighborCell::Inside(opt) = neigborhood.on_relative(&off) {
                    match opt {
                        None => {
                            new_mud.movement = off;
                            new_mud.velocity = MAX_VELOCITY.min(new_mud.velocity + GRAVITY);
                            // Reset side friction
                            new_mud.side_friction = get_side_friction();

                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_mud.movement = off;
                                // Apply some slowdown as if by friction of switching
                                new_mud.velocity = DEFAULT_VELOCITY.max(new_mud.velocity - SWITCH_SLOWDOWN);
                                // Reset side friction
                                new_mud.side_friction = get_side_friction();

                                break;
                            }
                        }
                    }
                }
            } else {
                new_mud.movement = Offset::zero();
                new_mud.velocity = DEFAULT_VELOCITY;
            }
        );

        ParticleChange::Changed(Some(Box::new(new_mud)))
    }
}

fn get_side_friction() -> u8 {
    get_value_around(DEFAULT_SIDE_FRICTION, FRICTION_OFF)
}
