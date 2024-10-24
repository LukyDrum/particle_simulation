use std::collections::LinkedList;

use rand::random;

use crate::particles::{get_near_color, Particle};
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
}

impl Smoke {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Smoke {
            color: get_near_color(COLOR),
            lifetime: get_value_around(DEFAULT_LIFETIME, LIFETIME_OFF),
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
        // Lifetime reached 0 => smoke is gone
        if self.lifetime == 0 {
            return ParticleChange::Changed(None);
        }

        // Clone smoke and decrease it's lifetime by 1
        let mut new_smoke = self.clone();
        new_smoke.lifetime -= 1;

        ParticleChange::Changed(Some(Box::new(new_smoke)))
    }
}
