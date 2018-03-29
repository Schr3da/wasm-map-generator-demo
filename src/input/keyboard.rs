use std::collections::{HashMap};
use std::process::{exit};
use sdl2::{Sdl};
use sdl2::{EventPump};
use sdl2::keyboard::{Keycode};
use sdl2::event::{Event};
use scenes::game::{Game};
use renderer::renderer::{Renderer};
use input::utils::{update_scroll_frame, update_render_target};

pub struct KeyboardControls{
    events: EventPump,
    keys: HashMap<Keycode, bool>,
}

impl KeyboardControls{

    pub fn new(context: &Sdl) -> KeyboardControls {
        KeyboardControls{
            events: context.event_pump().unwrap(),
            keys: HashMap::new(),
        }
    }

    pub fn update(&mut self, game: &mut Game, renderer: &mut Renderer){
        for event in self.events.poll_iter() {
            match event {
                Event::KeyDown{keycode, ..} => *(self.keys.entry(keycode.unwrap()).or_insert(true)) = true,
                Event::KeyUp{keycode: Some(Keycode::Escape), ..} => exit(1),
                Event::KeyUp{keycode: Some(Keycode::Num1), .. } => update_render_target(renderer),
                Event::KeyUp{keycode, ..} =>  *(self.keys.entry(keycode.unwrap()).or_insert(false)) = false,
                _ => {}
            }
        }

        update_scroll_frame(self, renderer, game);
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
