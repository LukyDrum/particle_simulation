use crate::particles::constants::*;
use crate::particles::{get_near_color, Particle};
use crate::Offset;

const COLOR: u32 = 0xFF474747;
const DENSITY: u8 = MAX_DENSITY;

#[derive(Clone)]
pub struct Rock {
    color: u32,
}

impl Rock {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Rock {
            color: get_near_color(COLOR),
        })
    }
}

impl Particle for Rock {
    fn get_name(&self) -> &str {
        "Rock"
    }

    fn get_color(&self) -> u32 {
        self.color
    }

    fn get_density(&self) -> u8 {
        DENSITY
    }

    fn is_moveable(&self) -> bool {
        false
    }

    fn is_solid(&self) -> bool {
        true
    }

    fn get_movement(&self) -> Offset {
        Offset::zero()
    }
}
