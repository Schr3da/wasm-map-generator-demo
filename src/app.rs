use sdl2;
use std::{thread};
use scenes::game::{Game};
use input;
use input::keyboard::{KeyboardControls};
use constants::{window};
use loader::font::{load_all_fonts};
use loader::assets::{FontManager};
use utils::main::{update, renderer};
use utils::canvas::{get_canvas};
use renderer::utils::{get_initial_position_frame};
use renderer::renderer::{RenderTarget, Renderer};

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
    let mut frame = get_initial_position_frame(RenderTarget::TopDown, &game);

    renderer(|is_init_done| {
        if is_init_done && !update(&mut keyboard_controlls) {
            thread::sleep(window::get_fps());
            return true;
        }

        let position_frame = input::utils::get_render_rect_for_keyboard(&keyboard_controlls, &mut frame, &render.get_frame());
        game.set_position_frame(position_frame);

        render.draw(&mut canvas, &texture_creator, &game);
        thread::sleep(window::get_fps());
        return true;
    });
}
