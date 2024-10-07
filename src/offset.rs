use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

        if !(x_diff.abs() == y_diff.abs() || x_diff == 0 || y_diff == 0) {
            return vec![];
        }

        // Find the direction of the difference - in case of difference of 0 we just leave a 1
        let x_sign = x_diff.signum();
        let y_sign = y_diff.signum();

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

    /// Get the the offset where both axis are scaled to magnitude of 1
    pub fn unit(&self) -> Offset {
        // Scaling to magnitude of 1 is the same as getting the sign
        let x = self.x.signum();
        let y = self.y.signum();

        Offset::new(x, y)
    }

    /// True if the offset is aiming down in accordance with the global coordinate system.
    pub fn is_down(&self) -> bool {
        self.y > 0
    }

    /// True if the offset is aiming right in accordance with the global coordinate system.
    pub fn is_right(&self) -> bool {
        self.x > 0
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

impl ops::Sub for Offset {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        Offset {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
