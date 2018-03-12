use std::collections::{HashMap};
use std::process::{exit};
use sdl2::{Sdl};
use sdl2::{EventPump};
use sdl2::keyboard::{Keycode};
use sdl2::event::{Event};
use renderer::renderer::{RenderTarget};

pub struct KeyboardControls{
    events: EventPump,
    keys: HashMap<Keycode, bool>,
    render_target: RenderTarget,
}

impl KeyboardControls{

    pub fn new(context: &Sdl) -> KeyboardControls {
        KeyboardControls{
            events: context.event_pump().unwrap(),
            keys: HashMap::new(),
            render_target: RenderTarget::TopDown,
        }
    }

    pub fn update(&mut self){
        self.handle_events();
    }

    fn handle_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::KeyDown{keycode: Some(Keycode::Escape), ..} => exit(1),
                Event::KeyDown{keycode, ..} => *(self.keys.entry(keycode.unwrap()).or_insert(true)) = true,
                Event::KeyUp{keycode, ..} =>  *(self.keys.entry(keycode.unwrap()).or_insert(false)) = false,
                _ => {}
            }
        }
    }

    pub fn toogle_render_target(&mut self) {
        match self.render_target {
            RenderTarget::TopDown => self.render_target = RenderTarget::ThirdPerson,
            RenderTarget::ThirdPerson => self.render_target = RenderTarget::TopDown,
        }
    }

    pub fn get_keys(&self) -> &HashMap<Keycode, bool>{
        return &self.keys;
    }

    pub fn get_value(&self, k: &Keycode) -> bool {
        match self.keys.get(k) {
            Some(v) => *v,
            _ => false
        }
    }
}
