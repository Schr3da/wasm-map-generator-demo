extern crate sdl2;
extern crate rand;
extern crate num;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

pub mod constants;
pub mod entities;
pub mod utils;
pub mod scenes;
pub mod light;
pub mod input;
pub mod generator;
pub mod loader;
pub mod app;

fn main() {
    app::run();
}
