use std::fmt;
use std::clone;

pub fn modulo(a: int, b: int) -> int {
    (((a % b) + b) % b)
}

pub struct Point {
    pub x: int,
    pub y: int,
}

impl fmt::Show for Point {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl clone::Clone for Point {
    fn clone(&self) -> Point {
        Point{ x: self.x, y: self.y }
    }
}
