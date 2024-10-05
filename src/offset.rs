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

    /// Returns all offsets between self and the other.
    /// Panics if the offsets are not in straight nice line.
    pub fn between(&self, other: &Offset) -> Vec<Offset> {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        if !(x_diff == y_diff || x_diff == 0 || y_diff == 0) {
            return vec![];
        }

        // Find the direction of the difference - in case of difference of 0 we just leave a 1
        let x_sign = if x_diff != 0 {
            x_diff / x_diff.abs()
        } else {
            1
        };
        let y_sign = if y_diff != 0 {
            y_diff / y_diff.abs()
        } else {
            1
        };

        // If the X difference is 0, move only on the Y line
        if x_diff == 0 {
            return (0..=y_diff.abs())
                .map(|s| Offset::new(self.x, self.y + s * y_sign))
                .collect();
        }

        // If the Y difference is 0, move only on the X line
        if y_diff == 0 {
            return (0..=x_diff.abs())
                .map(|s| Offset::new(self.x + s * x_sign, self.y))
                .collect();
        }

        // Else do combination of the both above
        (0..=x_diff.abs()) // Could also be y_diff (they are the same)
            .map(|s| Offset::new(self.x + s * x_sign, self.y + s * y_sign))
            .collect()
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
