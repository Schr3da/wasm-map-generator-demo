use sdl2::{Sdl};
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use utils;

pub fn get_canvas(context: Sdl) -> Canvas<Window> {
    utils::window::get_window(context)
        .into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build()
        .unwrap()
}
