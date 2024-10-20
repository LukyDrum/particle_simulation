use std::collections::LinkedList;

use dyn_clone::DynClone;
use rand::{thread_rng, Rng};

use crate::offset::Offset;

use super::Burnability;

pub type Neighborhood<'a> = Vec<Vec<&'a Option<Box<dyn Particle>>>>;

/// Similiar to Option.
/// Contains information if the particle has changed or not.
pub enum ParticleChange {
    /// Particle has changed to Some other, or to None (was destroyed)
    Changed(Option<Box<dyn Particle>>),
    /// No change
    None,
}

impl ParticleChange {
    /// Returns true if the variant is Changed
    pub fn has_changed(&self) -> bool {
        match self {
            ParticleChange::Changed(_) => true,
            ParticleChange::None => false,
        }
    }
}

/// Returns a color similiar to the color provided
pub fn get_near_color(color: u32) -> u32 {
    let off = thread_rng().gen_range(0..0xF);

    let mut fin_color = color + off;
    fin_color += off << 8;
    fin_color += off << 16;

    fin_color
}

// Needed for DynClone
dyn_clone::clone_trait_object!(Particle);

/// A trait that all particle types implement.
/// To create your own particle types implement this trait.
pub trait Particle: Send + Sync + DynClone {
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

    /// Returns the Burnability of this particle. By default is Burnability::None.
    fn get_burnability(&self) -> Burnability {
        Burnability::None
    }

    /// Returns a new state of the particle based on it's neighborhood.
    /// By default returns None, meaning no update of inner state
    fn update(&self, _neigborhood: Neighborhood) -> ParticleChange {
        ParticleChange::None
    }

    // Mutable

    /// Resets the particle velocity to the DEFAULT_VELOCITY.
    fn reset_velocity(&mut self) -> ();

    /// Applies the provided acceleration to the velocity of this particle.
    fn apply_acceleration(&mut self, acc: f32) -> ();
}
