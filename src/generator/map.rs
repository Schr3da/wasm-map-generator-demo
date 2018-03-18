use sdl2::rect::{Rect};
use std::collections::{HashMap};
use scenes::layer::{Layer, Renderable};
use constants::{layers};
use generator::{utils};
use entities::tile::{Tile};

pub struct Map {
    layers: HashMap<String, Layer<Tile>>,
}

impl Map {

    pub fn new() -> Self {
        let mut map = Map {layers: HashMap::new()};
        map.generate();
        map
    }

    pub fn generate(&mut self) {
        let height_map = utils::create_height_map();
        let length = height_map.size();

        let mut layer = utils::create_default_layer(length, length);

        utils::create_surface(&mut layer, &height_map);
        utils::create_vegetation(&mut layer);

        self.layers.insert(String::from(layers::MAP), layer);
    }

    pub fn get_layers(&self) -> &HashMap<String, Layer<Tile>> {
        &self.layers
    }

    pub fn get_frame(&self) -> Rect {
        self.layers[layers::MAP].get_frame()
    }
}
