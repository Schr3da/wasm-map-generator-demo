use std::vec::{Vec};
use sdl2::rect::{Rect};
use entities::entity::{Entity};

pub trait Renderable <T> where T: Entity {
    fn get_entities(&self) -> &Vec<Box<T>>;
    fn get_mut_entities(&mut self) -> &mut Vec<Box<T>>;
    fn add_entity(&mut self, e: Box<T>);
    fn get_frame(&self) -> Rect;
    fn get_size(&self) -> i32;
}

pub struct Layer<T> {
    entities: Vec<Box<T>>,
    frame: Rect
}

impl<T> Layer <T>{
    pub fn new(f: Rect) -> Layer<T> {
        Layer {
            entities: Vec::new(),
            frame: Rect::new(f.x(), f.y(), f.width(), f.height())
        }
    }
}

impl<T> Renderable<T> for Layer<T> where T: Entity {
    fn get_entities(&self) -> &Vec<Box<T>> {
        &self.entities
    }

    fn get_mut_entities(&mut self) -> &mut Vec<Box<T>> {
        &mut self.entities    
    }

    fn add_entity(&mut self, e: Box<T>) {
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
