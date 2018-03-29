use sdl2::render::{Canvas, RenderTarget, TextureCreator, Texture};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use sdl2::video::{WindowContext};
use renderer::top_down_utils::{create_map_texture, create_light_map_texture, get_scroll_position, get_player_frame, get_light_frame};
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
        self.draw_map(canvas, game, screen_frame); 
        self.draw_player(canvas, game); 
        self.draw_light(canvas, game, screen_frame);
    }

    fn draw_map<T>(&self, canvas: &mut Canvas<T>, game: &Game, screen_frame: Rect)  where T: RenderTarget {
        let (mx, my) = get_scroll_position(game);
        let src_map = Rect::new(mx , my, screen_frame.width(), screen_frame.height());
        canvas.copy(&self.map, src_map, screen_frame).unwrap();
    }

    fn draw_player<T>(&self, canvas: &mut Canvas<T>, game: &Game)  where T: RenderTarget {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let projected_frame = get_player_frame(game); 
        canvas.fill_rect(projected_frame).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    fn draw_light<T>(&self, canvas: &mut Canvas<T>, game: &Game, screen_frame: Rect) where T: RenderTarget {
        let mut projected_frame = get_light_frame(game);
        projected_frame.set_width(screen_frame.width());
        projected_frame.set_height(screen_frame.height());
        canvas.copy(&self.light, projected_frame, screen_frame).unwrap();
    }
}
