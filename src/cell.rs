use crate::particles::Particle;

/// Represents a cell in the Neighborhood. Can be either `Inside` with an Option of a particle, or `Outside` = outside of the grid of simulation.
#[derive(Clone)]
pub enum Cell {
    Inside(Option<Box<dyn Particle>>),
    Outside,
}

impl Cell {
    /// Returns true if `self` is `Inside`.
    pub fn is_inside(&self) -> bool {
        match self {
            Cell::Inside(_) => true,
            Cell::Outside => false,
        }
    }

    /// Returns true if `self` is `Outside`.
    pub fn is_outside(&self) -> bool {
        match self {
            Cell::Outside => true,
            Cell::Inside(_) => false,
        }
    }

    /// Returns true if `self` is `Inside` and contains an Option with Some.
    pub fn is_some(&self) -> bool {
        if let Cell::Inside(opt) = self {
            match opt {
                Some(_) => true,
                None => false,
            }
        } else {
            false
        }
    }

    /// Returns true if `self` is `Inside` and contains an Option with None.
    pub fn is_none(&self) -> bool {
        if let Cell::Inside(opt) = self {
            match opt {
                None => true,
                Some(_) => false,
            }
        } else {
            false
        }
    }
}
