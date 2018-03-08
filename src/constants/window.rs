use std::time::{Duration};
use sdl2::rect::{Rect};

pub static WIDTH: u32 =  1024;
pub static HEIGHT: u32 = 768;

pub fn get_fps() -> Duration {
    Duration::from_millis(16)
}

pub fn get_canvas_frame() -> Rect {
    Rect::new(0,0, WIDTH, HEIGHT)
}
