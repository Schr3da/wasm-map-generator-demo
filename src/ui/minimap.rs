use sdl2::rect::{Rect};
use constants::{window, map};

pub struct Minimap {
}

impl Minimap {

    pub fn new() -> Self {
        Minimap{}
    }

    pub fn get_frame(&self) -> Rect {
        Rect::new(window::WIDTH as i32 - map::MINI_MAP_SIZE as i32, 0,  map::MINI_MAP_SIZE, map::MINI_MAP_SIZE)
    }
}
