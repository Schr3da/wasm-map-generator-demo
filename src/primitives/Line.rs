use sdl2::rect::{Point};

pub struct Line {
    _p1: Point,
    _p2: Point
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        Line {
            _p1: p1,
            _p2: p2
        }
    }
}
