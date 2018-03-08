use sdl2::rect::{Rect};
use std::string::{String};
use std::collections::{HashMap};
use entities::entity::{Entity};
use entities::unit::{Unit};
use entities::utils::{create_default_unit_at_position};
use scenes::layer::{Layer};
use generator::map::{Map};
use constants::{tile};

pub struct Game {
    map: Map,
    player: Box<Unit>
}

impl Game {

    pub fn new() -> Game {
        Game {
            map: Map::new(),
            player: create_default_unit_at_position(0,0),
        }
    }

    pub fn init(&mut self) {
        self.map.generate();
    }

    pub fn get_map_layers(&self) -> &HashMap<String, Layer> {
        &self.map.get_layers()
    }

    pub fn set_player_position(&mut self, x: i32, y: i32) {
        self.player.set_position(x, y);
    }

    pub fn get_player_position(&self) -> (i32, i32) {
        self.player.get_position()
    }

    pub fn get_map_size(&self) -> (u32, u32) {
        let f = self.map.get_frame();
        (f.width(), f.height())
    }

    pub fn get_map_frame(&self) -> Rect {
        let (width, height) = self.get_map_size();
        let (px, py) = self.get_player_position();
        Rect::new(px, py, width * tile::WIDTH as u32, height * tile::HEIGHT as u32)
    }

    pub fn get_mini_map_frame(&self) -> Rect {
        self.map.get_mini_map_frame()
    }
}
