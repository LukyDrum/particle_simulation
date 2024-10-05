use rand::{random, thread_rng, Rng};

use crate::offset::Offset;

const DENSITY_MAX: u8 = 255;
const DEFAULT_VELOCITY: f32 = 1.0;
const GRAVITY: f32 = 0.1;

#[derive(Clone, Copy)]
pub struct Particle {
    pub color: u32,
    /// Higher density will fall through lower density. Set to 255 for absolutly solid particles.
    /// Gasses are near to 0, Fluids around 128, Solid particles at 255.
    pub density: u8,
    pub is_moveable: bool,
    pub velocity: f32,
    pub primary_offset: Offset,
    pub secondary_offsets: [Offset; 2],
}

// Implementations for creating different particle types
impl Particle {
    pub fn sand() -> Particle {
        Particle {
            color: Self::get_near_color(0x00E0E02D),
            density: DENSITY_MAX,
            is_moveable: true,
            velocity: DEFAULT_VELOCITY,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
        }
    }

    pub fn water() -> Particle {
        Particle {
            color: Self::get_near_color(0x001BB2E0),
            density: 128,
            is_moveable: true,
            velocity: DEFAULT_VELOCITY,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
        }
    }

    pub fn rock() -> Particle {
        Particle {
            color: Self::get_near_color(0x00909090),
            density: DENSITY_MAX,
            is_moveable: false,
            velocity: DEFAULT_VELOCITY,
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
        }
    }
}

impl Particle {
    pub fn reset_velocity(&mut self) -> () {
        self.velocity = DEFAULT_VELOCITY;
    }

    pub fn increment_velocity(&mut self) -> () {
        self.velocity += GRAVITY;
    }

    pub fn get_offsets(&self) -> Vec<Offset> {
        // Randomly choose the first of the secondary offsets
        let a: usize;
        let b: usize;

        if random() {
            a = 0;
            b = 1;
        } else {
            a = 1;
            b = 0;
        }

        let offset_a = self.secondary_offsets[a];
        let offset_b = self.secondary_offsets[b];

        // Add primary, A and B offsets multiplied by the velocity in order
        let mut offsets = Vec::with_capacity(self.velocity as usize * 3); // By 3 because there are 3 base offsets
        for base in [self.primary_offset, offset_a, offset_b] {
            for i in 1..(self.velocity as i32) {
                offsets.push(base * i);
            }
        }

        offsets
    }
}

impl Particle {
    // Get a color similiar to the provided one
    fn get_near_color(color: u32) -> u32 {
        let off = thread_rng().gen_range(0..0x15);

        let mut fin_color = color + off;
        fin_color += off << 8;
        fin_color += off << 16;

        fin_color
    }
}
