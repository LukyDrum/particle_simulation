use crate::offset::Offset;
use crate::Cell;

/// Hold cell in a 3x3 neighborhood. None means the cell is out of the grid.
pub struct Neighborhood<'a>(pub Vec<Vec<Option<&'a Cell>>>);

impl<'a> Neighborhood<'a> {
    pub fn up(&self) -> &Option<&Cell> {
        &self.0[0][1]
    }

    pub fn down(&self) -> &Option<&Cell> {
        &self.0[2][1]
    }

    pub fn left(&self) -> &Option<&Cell> {
        &self.0[1][0]
    }

    pub fn right(&self) -> &Option<&Cell> {
        &self.0[1][2]
    }

    pub fn up_left(&self) -> &Option<&Cell> {
        &self.0[0][0]
    }

    pub fn up_right(&self) -> &Option<&Cell> {
        &self.0[0][2]
    }

    pub fn down_left(&self) -> &Option<&Cell> {
        &self.0[2][0]
    }

    pub fn down_right(&self) -> &Option<&Cell> {
        &self.0[2][2]
    }

    pub fn center(&self) -> &Option<&Cell> {
        &self.0[1][1]
    }

    /// Returns the Option on the `offset` relative to the center of neighborhood.
    pub fn on_relative(&self, offset: &Offset) -> &Option<&Cell> {
        &self.0[(1 + offset.y) as usize][(1 + offset.x) as usize]
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<Option<&Cell>>>> {
        self.0.iter().flatten()
    }
}
