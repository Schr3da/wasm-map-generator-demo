use sdl2::{Sdl};
use sdl2::video::{Window};
use constants::window::{WIDTH, HEIGHT};

pub fn get_window(context: Sdl) -> Window{
    let video = context.video().unwrap();
    video.window("", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}
