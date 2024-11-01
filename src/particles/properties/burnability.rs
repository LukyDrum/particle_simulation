use crate::particles::Particle;
use crate::{Cell, Neighborhood};

use super::PropertyCheckResult;
use fastrand;

/// Describes how does the particle burn.
#[derive(Clone, Copy)]
pub enum Burnability {
    /// The particle is currently on fire, can light up other particles.
    /// Contains a durability parameter (How long before the particle burns down).
    IsBurning(u8),
    /// The particle can burn if next to fire.
    CanBurn,
    /// Not only the particle does not burn (like None) but also destroy other burning particles
    AntiBurn,
    /// The particle does not react to fire in any way.
    None,
}

impl Burnability {
    pub fn decreased_by(&self, value: u8) -> Burnability {
        match self {
            Self::IsBurning(time) => {
                let res = (*time as i32) - (value as i32);
                let res = res.max(0) as u8;
                Self::IsBurning(res)
            }
            _ => *self,
        }
    }

    /// Checks the burnability property of the particle and returns a `PropertyCheckResult` variant.
    ///
    /// Possible outcomes (can be more than one):
    ///     - 'Time' in the `IsBurning` variant ran out => Particle gets destroyed.
    ///     - 'Time' in the `IsBurning` variant was decreased by 1.
    ///     - If the particle `IsBurning` but there is mostly `AntiBurn` particles in its neighborhood, than the particle gets destroyed. (If the `check_antiburn` is set to true).
    ///     - If the particle is `CanBurn` but most of the particles are `IsBurning`, the particle will be set as `IsBurning` (If the `check_antiburn` is set to true, else at least 1 must be `IsBurning`).
    pub fn check<T: Particle>(
        particle: &mut T,
        neigborhood: &Neighborhood,
        default_burn_time: u8,
        check_antiburn: bool,
    ) -> PropertyCheckResult {
        let mut updated = false;
        if let Burnability::IsBurning(time) = particle.get_burnability() {
            if time == 0 {
                return PropertyCheckResult::Destroyed;
            } else {
                particle.set_burnability(particle.get_burnability().decreased_by(1));
                updated = true;
            }
        }

        // Check how many neighbors are IsBurning and how many are AntiBurn
        // If there is more of IsBurning => particle will keep burning
        // If there is more AntiBurn => particle will cease to be
        let mut burning_count = 0;
        let mut antiburn_count = 0;
        for opt in neigborhood.iter() {
            if let Cell::Inside(Some(neigh)) = opt {
                match neigh.get_burnability() {
                    Burnability::IsBurning(_) => burning_count += 1,
                    Burnability::AntiBurn => antiburn_count += 1,
                    _ => {}
                }
            }
        }

        match particle.get_burnability() {
            Burnability::IsBurning(_) => {
                if check_antiburn && antiburn_count > burning_count {
                    return PropertyCheckResult::Destroyed;
                }
            }
            // Set on fire based on check_antiburn and the counts
            Burnability::CanBurn => {
                let lower_bound = if check_antiburn { antiburn_count } else { 0 };

                if burning_count > lower_bound && fastrand::bool() {
                    particle.set_burnability(Burnability::IsBurning(default_burn_time));
                    updated = true;
                }
            }
            _ => {}
        }

        if updated {
            PropertyCheckResult::Updated
        } else {
            PropertyCheckResult::None
        }
    }
}
