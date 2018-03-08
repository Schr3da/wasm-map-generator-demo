use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use light::circle::{Circle};
use constants::player::{LIGHT_INTENSITY};

pub fn intersects_with_rect(r: &Rect, c: &Circle) -> bool {
    let p = c.get_point();
    let radius = c.get_radius();

    let dx = p.x() - i32::max(r.x(), i32::min(p.x(), r.x() + r.height() as i32));
    let dy = p.y() - i32::max(r.y(), i32::min(p.y(), r.y() + r.height() as i32));
    dx * dx + dy * dy < radius * radius
}

pub fn get_intensity(r: &Rect, c: &Circle) -> u8 {
    let p = c.get_point();
    let radius = c.get_radius();

    let dx = p.x() - i32::max(r.x(), i32::min(p.x(), r.x() + r.width() as i32));
    let dy = p.y() - i32::max(r.y(), i32::min(p.y(), r.y() + r.height() as i32));

    let value = (dx * dx + dy * dy) as f32 / (radius * radius) as f32 * LIGHT_INTENSITY as f32;
    u8::min(value as u8, LIGHT_INTENSITY)
}

pub fn calculate_light_intensity(r: &Rect, c: &Circle) -> Color {
    match intersects_with_rect(r, c) {
        true => Color::RGBA(0, 0, 0, get_intensity(r, c)),
        _ => Color::RGBA(0, 0, 0, LIGHT_INTENSITY)
    }
}
