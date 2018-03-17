use std::collections::{HashMap};
use std::string::{String};
use std::rc::{Rc};
use sdl2::rect::{Rect};
use sdl2::ttf::{Font};
use constants::{window};
use loader::assets::{FontDetails, FontManager};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (window::WIDTH as i32 - w) / 2;
    let cy = (window::HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn get_font_details() -> HashMap<String, FontDetails> {
    let mut dict = HashMap::new();
    dict.insert("normal".to_string(), FontDetails {path: "./../assets/DejaVuSans.ttf".to_string(), size: 32});
    dict.insert("bold".to_string(), FontDetails {path: "./../assets/DejaVuSansCondensed-Bold.ttf".to_string(), size: 32});
    dict
}

pub fn get_font_for_key<'l>(manager: &'l mut  FontManager, key: &'static str) -> Rc<Font<'l, 'static>> {
    let fonts = get_font_details();
    manager.load(&fonts.get(&key.to_string()).unwrap()).unwrap()
}

pub fn load_all_fonts(manager: &mut FontManager) {
    let fonts = get_font_details();
    for (_, font) in fonts.iter() {
        let _ = manager.load(&font);
    }
}
