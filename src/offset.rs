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

    pub fn zero() -> Offset {
        Offset { x: 0, y: 0 }
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

impl ops::Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Self::Output {
        Offset {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
