use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use std::cmp::{min, max};
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng};
use constants::{map, theme};
use generator::{diamond_square};
use scenes::layer::{Layer};
use entities::utils::{create_default_units_for_layer};

pub fn create_height_map() -> diamond_square::PixelMap<u8> {
    let buffer = diamond_square::construct(map::DETAILS);
    diamond_square::normalize_pixel_map(buffer)
}

pub fn create_default_layer(w: u32, h: u32) -> Layer {
    let mut layer = Layer::new(Rect::new(0, 0, w, h));
    create_default_units_for_layer(&mut layer, w as i32, h as i32);
    layer
}

pub fn get_range(v1: u32, v2: u32) -> Range<u32> {
    if v1 == v2 {
        Range::new(v1, v2+2)
    } else {
        Range::new(
            min(v1, v2),
            max(v1, v2)
        )
    }
}

pub fn get_random_color(colors: &[Color; 3]) -> Color {
    let range = get_range(0, colors.len() as u32);
    let mut rng = thread_rng();
    colors[range.ind_sample(&mut rng) as usize]
}

pub fn get_tile_color(hm: &diamond_square::PixelMap<u8>, x: i32, y: i32) -> Color  {
    match hm.get_pixel(x as u32, y as u32) {
        0...80 => Color::RGBA(8, 33, 51, 255),
        0...110 => Color::RGBA(18, 63, 102, 255),
        0...120 => Color::RGBA(28, 94, 153, 255),
        0...130 => Color::RGBA(32, 110, 179, 255),
        0...140 => Color::RGBA(37, 126, 204, 255),
        0...150 => Color::RGBA(41, 142, 230, 255),
        0...160 => Color::RGBA(46, 157, 255, 255),
        0...170 => Color::RGBA(77, 172, 255, 255),
        0...180 => Color::RGBA(128, 195, 255, 255),
        0...220 => theme::get_light_green(),
        0...255 => theme::get_green(),
        _ => theme::get_dark_blue(),
    }
}
