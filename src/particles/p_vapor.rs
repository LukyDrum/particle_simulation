use std::collections::LinkedList;

use rand::random;

use crate::particles::{get_near_color, Particle};
use crate::utility::get_value_around;
use crate::Offset;

use super::{ParticleChange, Water};

const COLOR: u32 = 0xFFE3E3E3;
const DENSITY: u8 = 16;
/// Default lifetime in number of updates
const DEFAULT_LIFETIME: u32 = 600;
const LIFETIME_OFF: u32 = 300;

#[derive(Clone)]
pub struct Vapor {
    color: u32,
    lifetime: u32,
}

impl Vapor {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Vapor {
            color: get_near_color(COLOR),
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
        })
    }
}

impl Particle for Vapor {
    fn get_name(&self) -> &str {
        "Vapor"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn _get_offsets(&self) -> LinkedList<Offset> {
        let mut lst = LinkedList::new();

        lst.push_back(Offset::new(0, -1));

        if random() {
            lst.push_back(Offset::new(1, 0));
            lst.push_back(Offset::new(-1, 0));
        } else {
            lst.push_back(Offset::new(-1, 0));
            lst.push_back(Offset::new(1, 0));
        }

        lst
    }

    fn is_moveable(&self) -> bool {
        true
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn update(&self, _neigborhood: super::Neighborhood) -> ParticleChange {
        // Lifetime reached 0 => vapor condenses to water
        if self.lifetime == 0 {
            return ParticleChange::Changed(Some(Water::new()));
        }

        // Clone vapor and decrease it's lifetime by 1
        let mut new_vapor = self.clone();
        new_vapor.lifetime -= 1;

        ParticleChange::Changed(Some(Box::new(new_vapor)))
    }
}
