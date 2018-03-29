use sdl2::pixels::{Color};
use sdl2::rect::{Rect, Point};
use entities::entity::{Entity};
use constants::{player};
use primitives::line::{Line};

pub struct Player {
    speed: f32,
    theta: f32,
    acceleration: f32,
    frame: Rect,
    velocity: Point,
    fov: Line,
    background_color: Color,
}

impl Player {
    pub fn new(f: Rect, b_color: Color) -> Player {
        let (b_r, b_g, b_b, b_a) = b_color.rgba();

        Player {
            speed: player::SPEED,
            theta: player::THETA,
            acceleration: player::ACCELERATION,
            velocity: player::get_velocity(),
            fov: player::get_fov(),
            frame: Rect::new(f.x(), f.y(), f.width(), f.height()),
            background_color: Color::RGBA(b_r, b_g, b_b, b_a),
        }
    }

    pub fn set_theta(&mut self, v: f32) {
        self.theta = v;
    }

    pub fn get_theta(&self) -> f32 {
        self.theta
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_acceleration(&self) -> f32 {
        self.acceleration
    }

    pub fn get_fov(&self) -> &Line {
        &self.fov
    }

    pub fn get_velocity(&self) -> &Point {
        &self.velocity
    }
}

impl Entity for Player {

    fn get_background(&self) -> Color {
        let (r, g, b, a) = self.background_color.rgba();
        Color::RGBA(r, g, b, a)
    }

    fn set_background(&mut self, c: Color) {
        self.background_color = c;
    }

    fn get_frame(&self) -> Rect {
        let f = self.frame;
        Rect::new(f.x(), f.y(), f.width(), f.height())
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
