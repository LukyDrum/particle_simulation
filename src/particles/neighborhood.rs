use crate::offset::Offset;

use super::Particle;

/// Represents a cell in the Neighborhood. Can be either `Inside` with an Option of a particle, or `Outside` = outside of the grid of simulation.
#[derive(Clone)]
pub enum NeighborCell<'a> {
    Inside(&'a Option<Box<dyn Particle>>),
    Outside,
}

impl<'a> NeighborCell<'a> {
    /// Returns true if `self` is `Inside`.
    pub fn is_inside(&self) -> bool {
        match self {
            NeighborCell::Inside(_) => true,
            NeighborCell::Outside => false,
        }
    }

    /// Returns true if `self` is `Outside`.
    pub fn is_outside(&self) -> bool {
        match self {
            NeighborCell::Outside => true,
            NeighborCell::Inside(_) => false,
        }
    }

    /// Returns true if `self` is `Inside` and contains an Option with Some.
    pub fn is_some(&self) -> bool {
        if let NeighborCell::Inside(opt) = self {
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
        if let NeighborCell::Inside(opt) = self {
            match opt {
                None => true,
                Some(_) => false,
            }
        } else {
            false
        }
    }
}

pub struct Neighborhood<'a>(pub Vec<Vec<NeighborCell<'a>>>);

impl<'a> Neighborhood<'a> {
    pub fn up(&self) -> &NeighborCell {
        &self.0[0][1]
    }

    pub fn down(&self) -> &NeighborCell {
        &self.0[2][1]
    }

    pub fn left(&self) -> &NeighborCell {
        &self.0[1][0]
    }

    pub fn right(&self) -> &NeighborCell {
        &self.0[1][2]
    }

    pub fn up_left(&self) -> &NeighborCell {
        &self.0[0][0]
    }

    pub fn up_right(&self) -> &NeighborCell {
        &self.0[0][2]
    }

    pub fn down_left(&self) -> &NeighborCell {
        &self.0[2][0]
    }

    pub fn down_right(&self) -> &NeighborCell {
        &self.0[2][2]
    }

    /// Returns the Option on the `offset` relative to the center of neighborhood.
    pub fn on_relative(&self, offset: &Offset) -> &NeighborCell {
        &self.0[(1 + offset.y) as usize][(1 + offset.x) as usize]
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<NeighborCell>>> {
        self.0.iter().flatten()
    }
}
