use crate::offset::Offset;
use crate::Cell;

pub struct Neighborhood<'a>(pub Vec<Vec<&'a Cell>>);

impl<'a> Neighborhood<'a> {
    pub fn up(&self) -> &Cell {
        &self.0[0][1]
    }

    pub fn down(&self) -> &Cell {
        &self.0[2][1]
    }

    pub fn left(&self) -> &Cell {
        &self.0[1][0]
    }

    pub fn right(&self) -> &Cell {
        &self.0[1][2]
    }

    pub fn up_left(&self) -> &Cell {
        &self.0[0][0]
    }

    pub fn up_right(&self) -> &Cell {
        &self.0[0][2]
    }

    pub fn down_left(&self) -> &Cell {
        &self.0[2][0]
    }

    pub fn down_right(&self) -> &Cell {
        &self.0[2][2]
    }

    /// Returns the Option on the `offset` relative to the center of neighborhood.
    pub fn on_relative(&self, offset: &Offset) -> &Cell {
        &self.0[(1 + offset.y) as usize][(1 + offset.x) as usize]
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<&Cell>>> {
        self.0.iter().flatten()
    }
}
