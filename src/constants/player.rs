use sdl2::rect::{Point};
use primitives::line::{Line};

pub static LIGHT_RADIUS: i32 = 260;
pub static LIGHT_INTENSITY: u8 = 180;
pub static SPEED: f32 = 0.1;
pub static ACCELERATION: f32 = 0.015;
pub static THETA: f32 = 0.0;

pub fn get_velocity() -> Point {
    Point::new(0, 0)
}

pub fn get_fov() -> Line {
    Line::new(
        Point::new(1, -1),
        Point::new(1, 1)
    )
}
