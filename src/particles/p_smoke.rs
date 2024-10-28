use std::collections::LinkedList;

use rand::random;

use super::constants::*;
use crate::particles::{get_near_color, NeighborCell, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::ParticleChange;

const COLOR: u32 = 0xFFB1B6BD;
const DENSITY: u8 = 20;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u32 = 750;
const LIFETIME_OFF: u32 = 300;

#[derive(Clone)]
pub struct Smoke {
    color: u32,
    lifetime: u32,
    movement: Offset,
}

impl Smoke {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Smoke {
            color: get_near_color(COLOR),
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
            movement: Offset::zero(),
        })
    }
}

impl Particle for Smoke {
    fn get_name(&self) -> &str {
        "Smoke"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn get_movement(&self) -> Offset {
        self.movement
    }

    fn update(&self, neigborhood: super::Neighborhood) -> ParticleChange {
        // Lifetime reached 0 => smoke is gone
        if self.lifetime == 0 {
            return ParticleChange::Changed(None);
        }

        // Clone smoke and decrease it's lifetime by 1
        let mut new_smoke = self.clone();
        new_smoke.lifetime -= 1;

        let x_dir = if fastrand::bool() { 1 } else { -1 };
        // Find new movement
        for_else!(
            for off in [Offset::new(0, -1), Offset::new(x_dir, 0), Offset::new(-x_dir, 0)] => {
                if let NeighborCell::Inside(opt) = neigborhood.on_relative(&off) {
                    match opt {
                        None => {
                            new_smoke.movement = off;
                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_smoke.movement = off;
                                break;
                            }
                        }
                    }
                }
            } else {
                new_smoke.movement = Offset::zero();
            }
        );

        ParticleChange::Changed(Some(Box::new(new_smoke)))
    }
}
