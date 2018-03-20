use sdl2::pixels::{Color};
use sdl2::rect::{Rect, Point};
use entities::entity::{Entity};

#[derive(Clone)]
pub struct Tile {
    frame: Rect,
    is_walkable: bool,
    background_color: Color,
}

impl Tile {

    pub fn new(f: Rect, b_color: Color) -> Tile {
        let (b_r, b_g, b_b, b_a) = b_color.rgba();

        Tile {
            frame: Rect::new(f.x(), f.y(), f.width(), f.height()),
            background_color: Color::RGBA(b_r, b_g, b_b, b_a),
            is_walkable: false,
        }
    }
    
    pub fn set_walkable(&mut self, v: bool) {
        self.is_walkable = v;
    }

    pub fn is_walkable(&self) -> bool {
        self.is_walkable
    }
}

impl Entity for Tile {

    fn get_background(&self) -> Color {
        let (r, g, b, a) = self.background_color.rgba();
        Color::RGBA(r, g, b, a)
    }

    fn set_background(&mut self, c: Color) {
        self.background_color = c;
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
