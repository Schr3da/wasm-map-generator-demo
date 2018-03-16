use sdl2;
use std::{thread};
use scenes::game::{Game};
use input::keyboard::{KeyboardControls};
use constants::{window};
use loader::font::{load_all_fonts};
use loader::assets::{FontManager};
use utils::main::{game_loop};
use utils::canvas::{get_canvas};
use renderer::renderer::{Renderer};

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let font_context = sdl2::ttf::init().unwrap();

    let mut keyboard_controlls = KeyboardControls::new(&sdl_context);
    let mut font_manager = FontManager::new(&font_context);
    load_all_fonts(&mut font_manager);

    let mut canvas = get_canvas(sdl_context);
    let texture_creator = canvas.texture_creator();

    let mut game = Game::new();
    let mut render = Renderer::new(&mut canvas, &texture_creator, &game);

    game_loop(|| {
        keyboard_controlls.update(&mut game, &mut render);
        render.draw(&mut canvas, &texture_creator, &game);
        thread::sleep(window::get_fps());
        return true;
    });
}
