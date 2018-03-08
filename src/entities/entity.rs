use sdl2::rect::{Rect};
use sdl2::pixels::{Color};

pub trait Entity {

    fn get_background(&self) -> Color;
    fn set_background(&mut self, c: Color);

    fn get_foreground(&self) -> Color;
    fn set_foreground(&mut self, c: Color);

    fn get_frame(&self) -> Rect;
    fn set_frame(&mut self, f: Rect);

    fn set_position(&mut self, x: i32, y: i32);
    fn get_position(&self) -> (i32, i32);

    fn set_width(&mut self, w: u32) {
        let mut f = self.get_frame();
        let h = f.height();
        f.resize(w, h);
        self.set_frame(f);
    }

    fn set_height(&mut self, h: u32) {
        let mut f = self.get_frame();
        let w = f.width();
        f.resize(w, h);
        self.set_frame(f);
    }

    fn set_size(&mut self, w: u32, h: u32) {
        let mut f = self.get_frame();
        f.resize(w, h);
        self.set_frame(f);
    }
}
