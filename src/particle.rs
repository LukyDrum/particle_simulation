use rand::{random, thread_rng, Rng};

use crate::offset::Offset;

const DENSITY_MAX: u8 = 255;
const DEFAULT_ENERGY: f32 = 1.0;
const MAX_ENERGY: f32 = 5.0;
const GRAVITY: f32 = 0.1;

// Colors
const SAND_COLOR: u32 = 0x00E0E02D;
const WATER_COLOR: u32 = 0x001BB2E0;
const ROCK_COLOR: u32 = 0x00909090;

#[derive(Clone, Copy)]
pub struct Particle {
    color: u32,
    color_function: fn(&Self) -> u32,
    /// Higher density will fall through lower density. Set to 255 for absolutly solid particles.
    /// Gasses are near to 0, Fluids around 128, Solid particles at 255.
    pub density: u8,
    pub is_moveable: bool,
    pub energy: f32,
    pub primary_offset: Offset,
    pub secondary_offsets: [Offset; 2],
}

// Implementations for creating different particle types
impl Particle {
    pub fn sand() -> Particle {
        Particle {
            color: Self::get_near_color(SAND_COLOR),
            color_function: |slf| slf.color,
            density: DENSITY_MAX,
            is_moveable: true,
            energy: DEFAULT_ENERGY,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
        }
    }

    pub fn water() -> Particle {
        // We want the water to be more white when moving fast.
        let color_function: fn(&Self) -> u32 = |slf| {
            if slf.energy >= 2.0 {
                return 0x00BAEEFF;
            }

            slf.color
        };

        Particle {
            color: Self::get_near_color(WATER_COLOR),
            color_function,
            density: 128,
            is_moveable: true,
            energy: DEFAULT_ENERGY,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
        }
    }

    pub fn rock() -> Particle {
        Particle {
            color: Self::get_near_color(ROCK_COLOR),
            color_function: |slf| slf.color,
            density: DENSITY_MAX,
            is_moveable: false,
            energy: DEFAULT_ENERGY,
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
        }
    }
}

impl Particle {
    pub fn reset_energy(&mut self) -> () {
        self.energy = DEFAULT_ENERGY;
    }

    pub fn increment_energy(&mut self) -> () {
        self.energy = MAX_ENERGY.min(self.energy + GRAVITY);
    }

    pub fn get_max_offsets(&self) -> Vec<Offset> {
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

        // Multiple the offsets by the velocity
        let v_int = self.energy as i32;
        vec![
            self.primary_offset * v_int,
            offset_a * v_int,
            offset_b * v_int,
        ]
    }

    /// Calls the color function of the particle, this lets particles change color based on it's parameters
    pub fn get_color(&self) -> u32 {
        (self.color_function)(self)
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
