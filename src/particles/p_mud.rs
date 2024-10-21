use std::collections::LinkedList;

use rand::random;

use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::ParticleChange;

const COLOR: u32 = 0xFF3D1812;
const DEFAULT_SIDE_FRICTION: u8 = 16;
const FRICTION_OFF: u8 = 4;

#[derive(Clone)]
pub struct Mud {
    velocity: f32,
    color: u32,
    /// Definies how much does the mud particle want to fall to the side. Actually falls when it reaches zero
    side_friction: u8,
}

impl Mud {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Mud {
            velocity: DEFAULT_VELOCITY,
            color: get_near_color(COLOR),
            side_friction: get_side_friction(),
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

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();
        let vel = self.velocity as i32;

        lst.push_back(Offset::new(0, 1) * vel);

        // If side friction is not yet 0, then dont try to fall to side
        if self.side_friction > 0 {
            return lst;
        }

        if random() {
            lst.push_back(Offset::new(1, 1) * vel);
            lst.push_back(Offset::new(-1, 1) * vel);
        } else {
            lst.push_back(Offset::new(-1, 1) * vel);
            lst.push_back(Offset::new(1, 1) * vel);
        }

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        true
    }

    fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    fn apply_acceleration(&mut self, acc: f32) -> () {
        self.velocity = (self.velocity + acc).clamp(DEFAULT_VELOCITY, MAX_VELOCITY);
    }

    fn update(&self, neigborhood: super::Neighborhood) -> ParticleChange {
        // Check bellow and if there is some particle then either reset or decrease current side friction.
        let opt_below = neigborhood[2][1];
        match opt_below {
            Some(_) => {
                let mut new_mud = self.clone();
                if new_mud.side_friction == 0 {
                    new_mud.side_friction = get_side_friction();
                } else {
                    new_mud.side_friction -= 1;
                }

                ParticleChange::Changed(Some(Box::new(new_mud)))
            }
            None => ParticleChange::None,
        }
    }
}

fn get_side_friction() -> u8 {
    get_value_around(DEFAULT_SIDE_FRICTION, FRICTION_OFF)
}
