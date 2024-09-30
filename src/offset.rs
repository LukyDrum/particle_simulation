use std::ops;

#[derive(Clone, Copy)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Offset {
        Offset { x, y }
    }
}

impl ops::Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
