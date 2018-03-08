use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use input::keyboard::{KeyboardControls};
use constants::{tile};

pub fn get_render_rect_for_keyboard(controls: &KeyboardControls, src_rect: &mut Rect, dest_rect: &Rect) -> Rect {

    let dest_width = (dest_rect.width()) as i32;
    let dest_height = (dest_rect.height()) as i32;

    for (key_code, value) in controls.get_keys().into_iter(){

        let x = src_rect.x();
        let y = src_rect.y();
        let src_width = src_rect.width() as i32;
        let src_height = src_rect.height() as i32;

        if key_code == &Keycode::Left && value == &true && x > 0 {
            src_rect.set_x(x - (tile::WIDTH) as i32);
        }

        if  key_code == &Keycode::Right && value == &true && src_width - x > dest_width {
            src_rect.set_x(x + (tile::WIDTH) as i32);
        }

        if key_code == &Keycode::Down && value == &true && dest_height + y < src_height {
            src_rect.set_y(y + (tile::WIDTH) as i32);
        }

        if key_code == &Keycode::Up && value == &true && y > 0 {
            src_rect.set_y(y - (tile::WIDTH) as i32);
        }

        if src_rect.x() < 0 {
            src_rect.set_x(0);
        }

        if src_rect.y() < 0 {
            src_rect.set_y(0);
        }
    }

    let new_src = Rect::new(src_rect.x(), src_rect.y(), dest_rect.width(), dest_rect.height());
    return new_src;
}
