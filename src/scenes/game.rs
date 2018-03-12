use sdl2::rect::{Rect};
use std::string::{String};
use std::collections::{HashMap};
use entities::entity::{Entity};
use entities::unit::{Unit};
use entities::utils::{create_default_unit_at_position};
use scenes::layer::{Layer};
use generator::map::{Map};
use ui::minimap::{Minimap};

pub struct Game {
    map: Map,
    minimap: Minimap,
    player: Box<Unit>,
    position_frame: Rect,
}

impl Game {

    pub fn new() -> Game {
        Game {
            map: Map::new(),
            minimap: Minimap::new(),
            player: create_default_unit_at_position(0,0),
            position_frame: Rect::new(0,0,0,0)
         }
    }

    pub fn get_map_layers(&self) -> &HashMap<String, Layer> {
        &self.map.get_layers()
    }

    pub fn set_position_frame(&mut self, frame: Rect) {
        self.position_frame = frame;
        self.set_player_position(frame.x(), frame.y());
    }

    pub fn get_position_frame(&self) -> Rect {
        self.position_frame
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
        Rect::new(px, py, width, height)
    }

    pub fn get_mini_map_frame(&self) -> Rect {
        self.minimap.get_frame()
    }
}
