use std::i32;

use rustc_hash::FxHashSet;

use crate::Offset;

/// A collection of Offsets where they form a continuoes plane.
pub struct Area {
    offsets: FxHashSet<Offset>,
    // Due to coordinate system with (0,0) in top-left, the highest point is the one with the smallest y-value
    highest_point: i32,
}

impl Area {
    pub fn new() -> Self {
        Area {
            offsets: FxHashSet::default(),
            highest_point: i32::max_value(),
        }
    }

    pub fn add(&mut self, offset: Offset) -> () {
        self.highest_point = self.highest_point.min(offset.y);
        self.offsets.insert(offset);
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Offset> {
        self.offsets.iter()
    }

    pub fn are_neighbors(offset_a: &Offset, offset_b: &Offset) -> bool {
        let a = *offset_a;
        let b = *offset_b;

        let diff = a - b;

        // Excludes the top-left and so on neighbors
        diff.x.abs() <= 1 && diff.y.abs() <= 1 && (diff.x == 0 || diff.y == 0)
    }

    pub fn depth(&self, offset: &Offset) -> i32 {
        offset.y - self.highest_point
    }
}
