use sdl2::rect::{Rect};
use std::collections::{HashMap};
use scenes::layer::{Layer, Renderable};
use constants::{layers, tile, map, window};
use generator::{utils};

pub struct Map {
    layers: HashMap<String, Layer>,
}

impl Map {

    pub fn new() -> Map {
        Map {
            layers: HashMap::new()
        }
    }

    pub fn generate(&mut self) {
        let height_map = utils::create_height_map();
        let length = height_map.size();

        let mut layer = utils::create_default_layer(length, length);

        for e in layer.get_mut_entities().iter_mut() {
            let (x, y) = e.get_position();
            e.set_position(x * tile::WIDTH as i32, y * tile::HEIGHT as i32);
            e.set_background(utils::get_tile_color(&height_map, x, y));
        }

        self.layers.insert(String::from(layers::MAP), layer);
    }

    pub fn get_layers(&self) -> &HashMap<String, Layer> {
        &self.layers
    }

    pub fn get_frame(&self) -> Rect {
        self.layers[layers::MAP].get_frame()
    }

    pub fn get_mini_map_frame(&self) -> Rect {
        Rect::new(window::WIDTH as i32 - map::MINI_MAP_SIZE as i32, 0,  map::MINI_MAP_SIZE, map::MINI_MAP_SIZE)
    }

}
