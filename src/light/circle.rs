use sdl2::rect::{Point};

pub struct Circle {
    p: Point,
    r: i32,
}

impl Circle {
    pub fn new(x: i32, y: i32, r: i32) -> Circle {
        Circle{p: Point::new(x, y), r}
    }

    pub fn get_point(&self) -> Point {
        self.p
    }

    pub fn get_radius(&self) -> i32 {
        self.r
    }
}
