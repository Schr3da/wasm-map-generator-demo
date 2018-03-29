use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use std::cmp::{min, max};
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng};
use constants::{map, theme};
use generator::{diamond_square};
use scenes::layer::{Renderable, Layer}; 
use entities::utils::{create_default_tiles_for_layer};
use entities::tile::{Tile};
use entities::entity::{Entity};

pub fn create_height_map() -> diamond_square::PixelMap<u8> {
    let buffer = diamond_square::construct(map::DETAILS);
    diamond_square::normalize_pixel_map(buffer)
}

pub fn create_default_layer(w: u32, h: u32) -> Layer<Tile> {
    let mut layer = Layer::new(Rect::new(0, 0, w, h));
    create_default_tiles_for_layer(&mut layer, w as i32, h as i32);
    layer
}

pub fn create_surface(l: &mut Layer<Tile>, hm: &diamond_square::PixelMap<u8>) {
    for e in l.get_mut_entities().iter_mut() {
        let (x, y) = e.get_position();
        let value = hm.get_pixel(x as u32, y as u32); 
            
        e.set_position(x, y);
        e.set_walkable(is_initial_walkable(value));
        e.set_background(get_tile_color(value));
    }
}

pub fn create_vegetation<'a>(l: &'a mut Layer<Tile>) {
    //Initial interested tiles
    let mut new_tiles: Vec<Box<Tile>> = l.get_entities().iter().filter(|e| {
        e.is_walkable()
    }).cloned().collect(); 
    
    new_tiles.iter_mut().for_each(|e| {
//        e.set_background(Color::RGB(0, 255, 0));   
    });

    new_tiles.iter_mut().for_each(|e| {
//        e.set_background(Color::RGB(255, 0, 0));   
    });

    let to_modify = l.get_mut_entities().iter_mut().filter(|e| e.is_walkable());
    for (i, e) in to_modify.enumerate() {
        if e.is_walkable() {
            e.set_background(new_tiles[i].get_background());
            e.set_walkable(new_tiles[i].is_walkable());
        }
    }
}

pub fn get_random_color(colors: &[Color; 3]) -> Color {
    let range = get_range(0, colors.len() as u32);
    let mut rng = thread_rng();
    colors[range.ind_sample(&mut rng) as usize]
}

fn get_range(v1: u32, v2: u32) -> Range<u32> {
    if v1 == v2 {
        Range::new(v1, v2+2)
    } else {
        Range::new(
            min(v1, v2),
            max(v1, v2)
        )
    }
}

fn is_initial_walkable(hm_value: u8) -> bool {
    match hm_value {
        0...160 => false,
        _ => true,
    }
}

fn get_tile_color(hm_value: u8) -> Color  {
    match hm_value {
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

