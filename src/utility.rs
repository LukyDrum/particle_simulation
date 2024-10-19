use std::ops::{Add, Sub};

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    thread_rng, Rng,
};

use crate::{frame::Frame, offset::Offset, particles::Particle};

pub fn draw_ui_to_frame(
    frame: &mut Frame,
    current_particle: &Box<dyn Particle>,
    indicator_size: usize,
) {
    for offset in get_offsets_for_square(&Offset::new(5, 5), indicator_size) {
        let _ = frame.draw_pixel(
            offset.x as usize,
            offset.y as usize,
            current_particle.get_color(),
        );
    }
}

pub fn get_offsets_for_square(center: &Offset, size: usize) -> Vec<Offset> {
    let size_half = (size / 2) as i32;

    let mut offsets = Vec::new();

    for x in (center.x - size_half)..(center.x + size_half) {
        for y in (center.y - size_half)..(center.y + size_half) {
            offsets.push(Offset::new(x, y));
        }
    }

    offsets
}

pub fn get_close_value<T>(middle: T, radius: T) -> T
where
    T: Add + Sub + PartialOrd + SampleUniform,
{
    let sum: T = middle + radius;
    thread_rng().gen_range(middle..=(middle + radius))
}
