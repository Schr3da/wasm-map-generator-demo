use sdl2;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas};
use sdl2::render::{TextureCreator};
use sdl2::video::{WindowContext};
use renderer::top_down::{TopDown};
use renderer::third_person::{ThirdPerson};
use utils::texture::{create_texture_target};
use constants::{window};
use scenes::game::{Game};

pub enum RenderTarget {
    ThirdPerson,
    TopDown,
}

pub struct Renderer<'a> {
    frame: Rect,
    target: RenderTarget,
    td_renderer: TopDown<'a>,
    tp_renderer: ThirdPerson,
}

impl<'a> Renderer<'a> {

    pub fn new<T>(canvas: &mut Canvas<T>, t_creator: &'a TextureCreator<WindowContext>, game: &Game) -> Self where T: sdl2::render::RenderTarget {
        Renderer {
            frame: Rect::new(0, 0, window::WIDTH, window::HEIGHT),
            target: RenderTarget::TopDown,
            td_renderer: TopDown::new(canvas, t_creator, game),
            tp_renderer: ThirdPerson::new(canvas, game)
        }
    }

    pub fn get_target(&self) -> RenderTarget {
        match self.target {
            RenderTarget::ThirdPerson => RenderTarget::ThirdPerson,
            RenderTarget::TopDown => RenderTarget::TopDown,
        }
    }

    pub fn set_target(&mut self, target: RenderTarget) {
        self.target = target;
    }

    pub fn get_frame(&self) -> Rect {
        Rect::new(self.frame.x(), self.frame.y(), self.frame.width(), self.frame.height())
    }

    pub fn draw<'l, T>(&mut self, canvas: &mut Canvas<T>, t_creator: &'a TextureCreator<WindowContext>, game: &Game) where T: sdl2::render::RenderTarget {
        let mut screen = create_texture_target(&t_creator, window::WIDTH, window::HEIGHT).unwrap();

        canvas.clear();
        canvas.with_texture_canvas(&mut screen, |texture| {
            texture.clear();
            match self.get_target() {
                RenderTarget::TopDown => self.td_renderer.draw(texture, game, self.get_frame()),
                RenderTarget::ThirdPerson => self.tp_renderer.draw(texture, game, self.get_frame()),
            }

            texture.copy(&self.td_renderer.get_map(), None, game.get_mini_map_frame()).unwrap();
        }).unwrap();

        canvas.copy(&screen, None, window::get_canvas_frame()).unwrap();
        canvas.present();
    }
}
