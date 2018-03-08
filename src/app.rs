use sdl2;
use std::{thread};
use scenes::game::{Game};
use input;
use input::keyboard::{KeyboardControls};
use constants::{window};
use loader::font::{load_all_fonts};
use loader::assets::{FontManager};
use utils::main::{update, renderer};
use utils::texture::{create_texture_target};
use utils::canvas::{get_canvas, create_map_texture, create_light_map_texture};

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let font_context = sdl2::ttf::init().unwrap();

    let mut keyboard_controlls = KeyboardControls::new(&sdl_context);
    let screen_rect = window::get_canvas_frame();

    let mut font_manager = FontManager::new(&font_context);
    load_all_fonts(&mut font_manager);

    let mut canvas = get_canvas(sdl_context);
    let texture_creator = canvas.texture_creator();

    let mut screen_texture = create_texture_target(&texture_creator, window::WIDTH, window::HEIGHT).unwrap();

    let mut game = Game::new();
    game.init();

    let mut frame = game.get_map_frame();
    frame.set_x(800);
    frame.set_y(800);

    let mini_map_frame = game.get_mini_map_frame();
    let map  = create_map_texture(&texture_creator, &game, &mut canvas);
    let light_map = create_light_map_texture(&texture_creator, &game, &mut canvas);

    renderer(|is_init_done| {
        if is_init_done && !update(&mut keyboard_controlls) {
            thread::sleep(window::get_fps());
            return true;
        }

        canvas.clear();

        let pos = input::utils::get_render_rect_for_keyboard(&keyboard_controlls, &mut frame, &screen_rect);
        game.set_player_position(pos.x(), pos.y());

        canvas.with_texture_canvas(&mut screen_texture, |texture| {
            texture.clear();
            texture.copy(&map, pos, window::get_canvas_frame()).unwrap();
            texture.copy(&light_map, None, window::get_canvas_frame()).unwrap();
            texture.copy(&map, None, mini_map_frame).unwrap();
        }).unwrap();

        canvas.copy(&screen_texture, None, window::get_canvas_frame()).unwrap();

        canvas.present();
        thread::sleep(window::get_fps());
        return true;
    });
}
