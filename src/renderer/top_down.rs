use sdl2::render::{Canvas, RenderTarget, TextureCreator, Texture};
use sdl2::rect::{Rect};
use sdl2::video::{WindowContext};
use renderer::top_down_utils::{create_map_texture, create_light_map_texture};
use scenes::game::{Game};

pub struct TopDown<'a> {
    map: Texture<'a>,
    light: Texture<'a>,
}

impl<'a> TopDown<'a>{

    pub fn new<T>(canvas: &mut Canvas<T>, t_creator: &'a TextureCreator<WindowContext>, game: &Game) -> Self where T: RenderTarget {
        TopDown {
            map: create_map_texture(&t_creator, &game, canvas),
            light: create_light_map_texture(&t_creator, &game, canvas),
        }
    }

    pub fn get_map(&self) -> &Texture<'a> {
        &self.map
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<T>, game: &Game, screen_frame: Rect) where T: RenderTarget {
        let (px, py) = game.get_player_position();
        let src_map = Rect::new(px, py, screen_frame.width(), screen_frame.height());

        canvas.copy(&self.map, src_map, screen_frame).unwrap();
        canvas.copy(&self.light, None, screen_frame).unwrap();
    }
}
