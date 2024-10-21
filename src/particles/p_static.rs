use std::collections::LinkedList;

use crate::particles::constants::*;
use crate::particles::Particle;
use crate::Offset;

const DENSITY: u8 = MAX_DENSITY;

#[derive(Clone)]
pub struct Static {
    color: u32,
}

impl Static {
    pub fn new(color: u32) -> Box<dyn Particle> {
        Box::new(Static { color })
    }
}

impl Particle for Static {
    fn get_name(&self) -> &str {
        "Static"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn get_max_offsets(&self) -> LinkedList<Offset> {
        LinkedList::new()
    }

    fn is_moveable(&self) -> bool {
        false
    }

    fn is_solid(&self) -> bool {
        true
    }
}
