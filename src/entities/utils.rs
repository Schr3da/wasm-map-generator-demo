use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use std::boxed::{Box};
use constants::{map};
use entities::tile::{Tile};
use entities::player::{Player};
use scenes::layer::{Layer, Renderable};

pub fn create_default_tiles_for_layer(layer: &mut Layer<Tile>,  gx: i32, gy: i32){
    let ignore = map::IGNORE_BORDER_PIXELS;
    for y in 0..gy {
        for x in 0..gx {
            if y > ignore && y < gy - ignore && x > ignore && x < gx - ignore {
                let entity = create_default_tile_at_position(x, y);
                layer.add_entity(entity);
            }
        }
    }
}

pub fn create_default_tile_at_position(x: i32, y: i32) -> Box<Tile>{
    Box::new(
        Tile::new(
            Rect::new(x, y, 1, 1),
            Color::RGBA(0, 0, 0, 0),
        )
    )
}

pub fn create_player_with_initial_position(x: i32, y: i32) -> Box<Player>{
    Box::new(
        Player::new(
            Rect::new(x, y, 1, 1),
            Color::RGBA(0, 0, 0, 0),
        )
    )
}
