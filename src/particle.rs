use rand::random;

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
            color: 0x00FFF32D,
            density: 255,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: 128,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn water() -> Particle {
        Particle {
            color: 0x001BB2F2,
            density: 128,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: 128,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn rock() -> Particle {
        Particle {
            color: 0x00909090,
            density: 255,
            is_moveable: false,
            acidity: Acidity::DoesDissolve,
            burnability: Burnability::None,
            durability: 4, // Takes 4 simulation steps for it to dissolve
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn smoke() -> Particle {
        Particle {
            color: 0x00C7C7C7,
            density: 1,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::None,
            durability: 128,
            primary_offset: Offset::new(0, -1),
            secondary_offsets: [Offset::new(-1, -1), Offset::new(1, -1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn acid() -> Particle {
        Particle {
            color: 0x003EF719,
            density: 130, // Higher than water
            is_moveable: true,
            acidity: Acidity::IsAcid,
            burnability: Burnability::None,
            durability: 128,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn wood() -> Particle {
        Particle {
            color: 0x00451C03,
            density: 255,
            is_moveable: false,
            acidity: Acidity::None,
            burnability: Burnability::DoesBurn,
            durability: 4, // Burns for 4 simulation steps, then is destroyed.
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn oil() -> Particle {
        Particle {
            color: 0x00857110,
            density: 126, // Lower than water
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::DoesBurn,
            durability: 2, // Burns for 2 simulation steps, then is destroyed.
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn fire() -> Particle {
        Particle {
            color: 0x00FF0000,
            density: 255,
            is_moveable: true,
            acidity: Acidity::None,
            burnability: Burnability::IsBurning,
            durability: 4,
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
