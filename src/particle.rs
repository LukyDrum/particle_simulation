use rand::{random, thread_rng, Rng};

use crate::offset::Offset;

#[derive(Clone, Copy, PartialEq)]
pub enum Acidity {
    None,
    IsAcid,
    DoesDissolve,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Burnability {
    None,
    IsBurning,
    DoesBurn,
}

const DENSITY_MAX: u8 = 255;
const DURABILITY_MAX: u8 = 255;

/// Basic particle types:
///
/// Sand - just falls
///
/// Water - spreads to side
///
/// Rock - is immoveable but can dissolve in acidic particles
///
/// Smoke - raises opposite to water
///
/// Acid - denser than water, can dissolve rock
///
/// Wood - like rock, but does not dissolve in acid and is flameable
///
/// Oil - like water, but less dense and burns
///
/// Fire - like sand, but is burning
#[derive(Clone, Copy)]
pub struct Particle {
    pub color: u32,
    /// Higher density will fall through lower density. Set to 255 for absolutly solid particles.
    /// Gasses are near to 0, Fluids around 128, Solid particles at 255.
    pub density: u8,
    pub is_moveable: bool,
    /// Decides if it will dissolve other particle or be dissolved, or None
    pub acidity: Acidity,
    /// Decides if it will burn other particle or be burned, or None
    pub burnability: Burnability,
    /// Fire and acid can decrease durability, if reaches 0 the particle will be destroyed.
    pub durability: u8, // It makes stuff burn and dissolve at different rates. It only applies if they have the correct enum variants.
    pub primary_offset: Offset,
    pub secondary_offsets: [Offset; 2],
    pub ternary_offsets: [Offset; 2],
    pub was_update: bool, // This is needed for the simulation implementation
}

// Implementations for creating different particle types
impl Particle {
    pub fn sand() -> Particle {
        Particle {
            color: Self::get_near_color(0x00E0E02D),
            density: DENSITY_MAX,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: DURABILITY_MAX,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn water() -> Particle {
        Particle {
            color: Self::get_near_color(0x001BB2E0),
            density: 128,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: DURABILITY_MAX,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn rock() -> Particle {
        Particle {
            color: Self::get_near_color(0x00909090),
            density: DENSITY_MAX,
            is_moveable: false,
            acidity: Acidity::DoesDissolve,
            burnability: Burnability::None,
            durability: 8, // Takes N simulation steps for it to dissolve
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn smoke() -> Particle {
        Particle {
            color: Self::get_near_color(0x00C7C7C7),
            density: 1,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: DURABILITY_MAX,
            primary_offset: Offset::new(0, -1),
            secondary_offsets: [Offset::new(-1, -1), Offset::new(1, -1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn acid() -> Particle {
        Particle {
            color: Self::get_near_color(0x003EE219),
            density: 130, // Higher than water
            is_moveable: true,
            acidity: Acidity::IsAcid,
            burnability: Burnability::None,
            durability: DURABILITY_MAX,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn wood() -> Particle {
        Particle {
            color: Self::get_near_color(0x00451C03),
            density: DENSITY_MAX,
            is_moveable: false,
            acidity: Acidity::None,
            burnability: Burnability::DoesBurn,
            durability: 16, // Burns for N simulation steps, then is destroyed.
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn oil() -> Particle {
        Particle {
            color: Self::get_near_color(0x00857110),
            density: 126, // Lower than water
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::DoesBurn,
            durability: 6, // Burns for N simulation steps, then is destroyed.
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn fire() -> Particle {
        Particle {
            color: 0x00E00000,
            density: DENSITY_MAX,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::IsBurning,
            durability: 8,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(0, 0), Offset::new(0, 0)],
            was_update: false,
        }
    }
}

impl Particle {
    pub fn get_offsets_to_try(&self) -> Vec<Offset> {
        let mut offs = vec![self.primary_offset];

        let a: usize;
        let b: usize;

        if random() {
            a = 0;
            b = 1;
        } else {
            a = 1;
            b = 0;
        }

        offs.push(self.secondary_offsets[a]);
        offs.push(self.secondary_offsets[b]);
        offs.push(self.ternary_offsets[a]);
        offs.push(self.ternary_offsets[b]);

        offs
    }
}

impl Particle {
    fn get_near_color(color: u32) -> u32 {
        // let off_r = thread_rng().gen_range(0..0x10);
        // let off_g = thread_rng().gen_range(0..0x10);
        // let off_b = thread_rng().gen_range(0..0x10);
        let off = thread_rng().gen_range(0..0x15);

        let mut fin_color = color + off;
        fin_color += off << 8;
        fin_color += off << 16;

        fin_color
    }
}
