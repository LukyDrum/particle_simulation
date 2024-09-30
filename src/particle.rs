use rand::random;

use crate::offset::Offset;

#[derive(Clone, Copy)]
pub struct Particle {
    pub color: u32,
    pub is_solid: bool,
    pub is_moveable: bool,
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
            is_solid: true,
            is_moveable: true,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::zero(), Offset::zero()],
            was_update: false,
        }
    }

    pub fn water() -> Particle {
        Particle {
            color: 0x001BB2F2,
            is_solid: false,
            is_moveable: true,
            primary_offset: Offset::new(0, 1),
            secondary_offsets: [Offset::new(-1, 1), Offset::new(1, 1)],
            ternary_offsets: [Offset::new(-1, 0), Offset::new(1, 0)],
            was_update: false,
        }
    }

    pub fn rock() -> Particle {
        Particle {
            color: 0x00909090,
            is_solid: true,
            is_moveable: false,
            primary_offset: Offset::zero(),
            secondary_offsets: [Offset::zero(), Offset::zero()],
            ternary_offsets: [Offset::zero(), Offset::zero()],
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
