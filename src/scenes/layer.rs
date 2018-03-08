use std::vec::{Vec};
use sdl2::rect::{Rect};
use entities::entity::{Entity};

pub trait Renderable {
    fn get_entities(&self) -> &Vec<Box<Entity>>;
    fn get_mut_entities(&mut self) -> &mut Vec<Box<Entity>>;
    fn add_entity(&mut self, e: Box<Entity>);
    fn get_frame(&self) -> Rect;
    fn get_size(&self) -> i32;
}

pub struct Layer {
    entities: Vec<Box<Entity>>,
    frame: Rect
}

impl Layer {
    pub fn new(f: Rect) -> Layer {
        Layer {
            entities: Vec::new(),
            frame: Rect::new(f.x(), f.y(), f.width(), f.height())
        }
    }
}

impl Renderable for Layer {
    fn get_entities(&self) -> &Vec<Box<Entity>> {
        &self.entities
    }

    fn get_mut_entities(&mut self) -> &mut Vec<Box<Entity>> {
        &mut self.entities    
    }

    fn add_entity(&mut self, e: Box<Entity>) {
        self.entities.push(e);
    }

    fn get_frame(&self) -> Rect {
        let f = self.frame;
        Rect::new(f.x(), f.y(), f.width(), f.height())
    }

    fn get_size(&self) -> i32 {
        self.entities.capacity() as i32
    }
}
