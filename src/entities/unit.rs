use sdl2::pixels::{Color};
use sdl2::rect::{Rect, Point};
use entities::entity::{Entity};

pub struct Unit {
    frame: Rect,
    background_color: Color,
    sign: String,
    foreground_color: Color,
    sign_size: i32,
}

impl Unit {

    pub fn new(f: Rect, b_color: Color, f_color: Color, s: &str, size: i32) -> Unit {
        let (f_r, f_g, f_b, f_a) = f_color.rgba();
        let (b_r, b_g, b_b, b_a) = b_color.rgba();

        Unit {
            frame: Rect::new(f.x(), f.y(), f.width(), f.height()),
            foreground_color: Color::RGBA(f_r, f_g, f_b, f_a),
            background_color: Color::RGBA(b_r, b_g, b_b, b_a),
            sign: String::from(s),
            sign_size: size
        }
    }

    pub fn get_sign(&self) -> &String {
        &self.sign
    }

    pub fn get_sign_size(&self) -> &i32 {
        &self.sign_size
    }
}

impl Entity for Unit {

    fn get_background(&self) -> Color {
        let (r, g, b, a) = self.background_color.rgba();
        Color::RGBA(r, g, b, a)
    }

    fn set_background(&mut self, c: Color) {
        self.background_color = c;
    }

    fn get_foreground(&self) -> Color {
        let (r, g, b, a) = self.foreground_color.rgba();
        Color::RGBA(r, g, b, a)
    }

    fn set_foreground(&mut self, c: Color) {
        self.foreground_color = c;
    }

    fn get_frame(&self) -> Rect {
        self.frame
    }

    fn set_frame(&mut self, f: Rect) {
        self.frame = f;
    }

    fn set_position(&mut self, x: i32, y: i32) {
        let mut f = self.get_frame();
        f.reposition(Point::new(x, y));
        self.frame = f;
    }

    fn get_position(&self) -> (i32, i32) {
        return (self.frame.x(), self.frame.y());
    }

}
