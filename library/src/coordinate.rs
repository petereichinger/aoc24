use std::ops::{Add, Mul};

#[derive(Clone, Copy, Default, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Coord {
    type Output = Coord;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<Coord> for isize {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
