use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use std::boxed::{Box};
use constants::{tile, map};
use entities::unit::{Unit};
use scenes::layer::{Layer, Renderable};

pub fn create_default_units_for_layer(layer: &mut Layer,  gx: i32, gy: i32) {
    let ignore = map::IGNORE_BORDER_PIXELS;
    for y in 0..gy {
        for x in 0..gx {
            if y > ignore && y < gy - ignore && x > ignore && x < gx - ignore {
                let entity = create_default_unit_at_position(x, y);
                layer.add_entity(entity);
            }
        }
    }
}

pub fn create_default_unit_at_position(x: i32, y: i32) -> Box<Unit>{
    Box::new(
        Unit::new(
            Rect::new(
                x,
                y,
                tile::WIDTH,
                tile::HEIGHT
            ),
            Color::RGBA(0, 0, 0, 0),
            Color::RGBA(0, 0, 0, 0),
            "-",
            tile::FONT_SIZE
        )
    )
}
