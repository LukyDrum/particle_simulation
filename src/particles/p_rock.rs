use crate::particles::constants::*;
use crate::particles::Particle;
use crate::{Color, Offset};

const COLOR: u32 = 0x474747;
const DENSITY: u8 = MAX_DENSITY;

#[derive(Clone)]
pub struct Rock {
    color: Color,
}

impl Rock {
    pub fn new() -> Box<dyn Particle> {
        Box::new(Rock {
            color: Color::hex(COLOR).similiar(),
        })
    }
}

impl Particle for Rock {
    fn get_name(&self) -> &str {
        "Rock"
    }

    fn get_color(&self) -> &Color {
        &self.color
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
