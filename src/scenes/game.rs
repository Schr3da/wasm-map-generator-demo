use sdl2::rect::{Rect};
use std::string::{String};
use std::collections::{HashMap};
use entities::entity::{Entity};
use entities::player::{Player};
use entities::utils::{create_player_with_initial_position};
use scenes::layer::{Layer};
use renderer::utils::{set_initial_scroll};
use generator::map::{Map};
use ui::minimap::{Minimap};
use entities::tile::{Tile};

pub struct Game {
    map: Map,
    minimap: Minimap,
    player: Box<Player>,
    frame: Rect,
}

impl Game {

    pub fn new() -> Game {
        let mut game = Game {
            map: Map::new(),
            minimap: Minimap::new(),
            player: create_player_with_initial_position(0, 0),
            frame: Rect::new(0, 0, 0, 0)
        };

        set_initial_scroll(&mut game);
        game
    }

    pub fn get_map_layers(&self) -> &HashMap<String, Layer<Tile>> {
        &self.map.get_layers()
    }

    pub fn set_scroll_frame(&mut self, frame: Rect) {
        self.frame = frame;
    }

    pub fn get_scroll_frame(&self) -> Rect {
        self.frame
    }

    pub fn get_player(&self) -> &Player {
        &self.player
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
        let (px, py) = self.player.get_position();
        Rect::new(px, py, width, height)
    }

    pub fn get_mini_map_frame(&self) -> Rect {
        self.minimap.get_frame()
    }
}
