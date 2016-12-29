use std::fmt;

pub fn modulo(a: i32, b: i32) -> i32 {
    (((a % b) + b) % b)
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

//impl fmt::Show for Point {
//    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "Point({}, {})", self.x, self.y)
//    }
//}
//
//impl clone::Clone for Point {
//    fn clone(&self) -> Point {
//        Point{ x: self.x, y: self.y }
//    }
//}
