use std::collections::LinkedList;

use rand::{thread_rng, Rng};

use crate::offset::Offset;

pub const DEFAULT_VELOCITY: f32 = 1.0;
pub const MAX_VELOCITY: f32 = 5.0;
pub const GRAVITY: f32 = 0.1;
pub const MAX_DENSITY: u8 = 255;

/// Returns a color similiar to the color provided
pub fn get_near_color(color: u32) -> u32 {
    let off = thread_rng().gen_range(0..0x15);

    let mut fin_color = color + off;
    fin_color += off << 8;
    fin_color += off << 16;

    fin_color
}

/// A trait that all particle types implement.
/// To create your own particle types implement this trait.
pub trait Particle {
    /// Creates a new instance of this particle.
    // fn new() -> Self;

    // Immutable

    /// Returns the color of the particle.
    fn get_color(&self) -> u32;

    /// Returns the density of this particle.
    /// The returned number is an 8bit unsigned integer (0-255).
    fn get_density(&self) -> u8;

    /// Returns the current velocity of this particle.
    fn get_velocity(&self) -> f32;

    /// Returns a list of the maximum offsets to which the particle would like to move to.
    /// Example: A maximum offset of (5, 0) means that the particle would like to move 5 positions to right.
    fn get_max_offsets(&self) -> LinkedList<Offset>;

    /// Returns true if the particle is moveable (can move).
    fn is_moveable(&self) -> bool;

    /// Returns true if the particle is completly solid (Example: rock).
    fn is_solid(&self) -> bool;

    // Mutable

    /// Resets the particle velocity to the DEFAULT_VELOCITY.
    fn reset_velocity(&mut self) -> ();

    /// Applies gravity to the velocity of this particle.
    fn apply_gravity(&mut self) -> ();
}
