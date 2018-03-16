use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use input::keyboard::{KeyboardControls};
use scenes::game::{Game};
use renderer::renderer::{Renderer};
use renderer::utils::{toogle_render_target};
use constants::{tile};

pub fn update_render_target(renderer: &mut Renderer) {
    let prev_render_target = renderer.get_target();
    let next_render_target = toogle_render_target(prev_render_target);
    renderer.set_target(next_render_target);
}

pub fn update_player_position(controls: &KeyboardControls, renderer: &mut Renderer, game: &mut Game, ) {
    let prev_position_frame = game.get_position_frame();
    let next_position_frame = get_render_rect_for_keyboard(controls, &prev_position_frame, &renderer.get_frame());
    game.set_position_frame(next_position_frame);
}

fn get_render_rect_for_keyboard(controls: &KeyboardControls, src_rect: &Rect, dest_rect: &Rect) -> Rect {
    let dest_width = (dest_rect.width()) as i32;
    let dest_height = (dest_rect.height()) as i32;
    let mut new_src_rect = Rect::new(src_rect.x(), src_rect.y(), src_rect.width(), src_rect.height());

    for (key_code, value) in controls.get_keys().into_iter(){

        let x = src_rect.x();
        let y = src_rect.y();
        let src_width = src_rect.width() as i32;
        let src_height = src_rect.height() as i32;

        if key_code == &Keycode::Left && value == &true && x > 0 {
            new_src_rect.set_x(x - (tile::WIDTH) as i32);
        }

        if  key_code == &Keycode::Right && value == &true && src_width - x > dest_width {
            new_src_rect.set_x(x + (tile::WIDTH) as i32);
        }

        if key_code == &Keycode::Down && value == &true && dest_height + y < src_height {
            new_src_rect.set_y(y + (tile::WIDTH) as i32);
        }

        if key_code == &Keycode::Up && value == &true && y > 0 {
            new_src_rect.set_y(y - (tile::WIDTH) as i32);
        }

        if src_rect.x() < 0 {
            new_src_rect.set_x(0);
        }

        if src_rect.y() < 0 {
            new_src_rect.set_y(0);
        }
    }

    return new_src_rect
}
