use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use scenes::game::{Game};

pub struct ThirdPerson {

}

impl ThirdPerson {

    pub fn new<T>(_canvas: &mut Canvas<T>, _game: &Game) -> Self where T: RenderTarget {
        ThirdPerson {

        }
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<T>, _game: &Game, _screen_frame: Rect) where T: RenderTarget {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
}
