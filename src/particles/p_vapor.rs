use crate::particles::Particle;
use crate::utility::get_value_around;
use crate::{Cell, Color, Neighborhood, Offset};

use super::{ParticleChange, Water};

const COLOR: u32 = 0xE3E3E3;
const DENSITY: u8 = 16;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u32 = 600;
const LIFETIME_OFF: u32 = 300;

#[derive(Clone)]
pub struct Vapor {
    color: Color,
    lifetime: u32,
    movement: Offset,
}

impl Vapor {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Vapor {
            color: Color::hex(COLOR).similiar(),
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
            movement: Offset::zero(),
        })
    }
}

impl Particle for Vapor {
    fn get_name(&self) -> &str {
        "Vapor"
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn get_movement(&self) -> Offset {
        self.movement
    }

    fn update(&self, neigborhood: Neighborhood) -> ParticleChange {
        // Lifetime reached 0 => vapor condenses to water
        if self.lifetime == 0 {
            return ParticleChange::Changed(Some(Water::new()));
        }

        // Clone vapor and decrease it's lifetime by 1
        let mut new_vapor = self.clone();
        new_vapor.lifetime -= 1;

        let x_dir = if fastrand::bool() { 1 } else { -1 };
        // Find new movement
        for_else!(
            for off in [Offset::new(0, -1), Offset::new(x_dir, 0), Offset::new(-x_dir, 0)] => {
                if let Some(cell) = neigborhood.on_relative(&off) {
                    match cell.get_particle() {
                        None => {
                            new_vapor.movement = off;
                            break;
                        }
                        Some(other) => {
                            if self.can_switch_with(other) {
                                new_vapor.movement = off;
                                break;
                            }
                        }
                    }
                }
            } else {
                new_vapor.movement = Offset::zero();
            }
        );

        ParticleChange::Changed(Some(Box::new(new_vapor)))
    }
}
