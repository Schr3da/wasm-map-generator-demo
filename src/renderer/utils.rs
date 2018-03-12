use sdl2::rect::{Rect};
use scenes::game::{Game};
use renderer::renderer::{RenderTarget};
use constants::{tile};

pub fn get_initial_position_frame(target: RenderTarget, g: &Game) -> Rect {
    let frame = g.get_map_frame();
    match target {
        RenderTarget::TopDown => Rect::new(800, 800, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT),
        RenderTarget::ThirdPerson => Rect::new(0, 0, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT),
    }
}

pub fn get_position_frame(target: RenderTarget, g: &Game) -> Rect {
    let frame = g.get_map_frame();
    let (x,y) = g.get_player_position();

    match target {
        RenderTarget::TopDown => Rect::new(x, y, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT),
        RenderTarget::ThirdPerson => Rect::new(x, y, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT),
    }
}
