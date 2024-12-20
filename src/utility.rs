use std::ops::{Add, Sub};

use rand::{distributions::uniform::SampleUniform, thread_rng, Rng};

use crate::offset::Offset;

pub fn get_offsets_for_square(center: &Offset, size: u32) -> Vec<Offset> {
    let size_half = (size / 2) as i32;

    let mut offsets = Vec::new();

    for x in (center.x - size_half)..(center.x + size_half) {
        for y in (center.y - size_half)..(center.y + size_half) {
            offsets.push(Offset::new(x, y));
        }
    }

    offsets
}

/// Returns a random value inside the radius with a center in middle.
pub fn get_value_around<T>(middle: T, radius: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + SampleUniform + PartialOrd + Copy,
{
    thread_rng().gen_range((middle - radius)..=(middle + radius))
}

/// Takes a for loop with an else branch. The else branch is executed if the for loop finishes all of its loops.
///
/// # Example:
/// ```rust
///     use particle_simulation::for_else;
///
///     for_else!(
///         for x in 1..10 => {
///             if x == 5 {
///                 break;
///             }
///         } else {
///             unreachable!();
///         }
///     );
/// ```
#[macro_export]
macro_rules! for_else {
    (for $var:ident in $collection:expr => $for_block:block else $else_block:block) => {
        #[allow(unused)]
        let mut flag = false;
        for $var in $collection {
            flag = false;

            $for_block;

            flag = true;
        }
        if flag {
            $else_block
        }
    };
}
pub use for_else;
