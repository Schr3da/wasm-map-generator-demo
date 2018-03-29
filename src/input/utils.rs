use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use input::keyboard::{KeyboardControls};
use scenes::game::{Game};
use renderer::renderer::{Renderer};
use renderer::utils::{toogle_render_target};
use constants::{map};

pub fn update_render_target(renderer: &mut Renderer) {
    let prev_render_target = renderer.get_target();
    let next_render_target = toogle_render_target(prev_render_target);
    renderer.set_target(next_render_target);
}

pub fn update_scroll_frame(controls: &KeyboardControls, _renderer: &mut Renderer, game: &mut Game, ) {
    let prev_scroll = game.get_scroll_frame();
    let next_scroll = handle_keyboard(controls, &prev_scroll);
    game.set_scroll_frame(next_scroll);
}

fn handle_keyboard(controls: &KeyboardControls, src_rect: &Rect) -> Rect {
    let mut new_src_rect = Rect::new(src_rect.x(), src_rect.y(), src_rect.width(), src_rect.height());
    let speed = 1;

    for (key_code, value) in controls.get_keys().into_iter(){

        let x = src_rect.x();
        let y = src_rect.y();
        let src_width = src_rect.width() as i32;
        let src_height = src_rect.height() as i32;

        if key_code == &Keycode::Left && value == &true && x - speed  > map::IGNORE_BORDER_PIXELS {
            new_src_rect.set_x(x - speed);
        }

        if  key_code == &Keycode::Right && value == &true && x + speed < src_width - (map::IGNORE_BORDER_PIXELS * 2) as i32 {
            new_src_rect.set_x(x + speed);
        }

        if key_code == &Keycode::Down && value == &true && y + speed < src_height - (map::IGNORE_BORDER_PIXELS * 2) as i32 {
            new_src_rect.set_y(y + speed);
        }

        if key_code == &Keycode::Up && value == &true && y - speed > map::IGNORE_BORDER_PIXELS {
            new_src_rect.set_y(y - speed);
        }

        if src_rect.x() < map::IGNORE_BORDER_PIXELS {
            new_src_rect.set_x(map::IGNORE_BORDER_PIXELS);
        }

        if src_rect.y() < map::IGNORE_BORDER_PIXELS {
            new_src_rect.set_y(map::IGNORE_BORDER_PIXELS);
        }
    }

    return new_src_rect
}
