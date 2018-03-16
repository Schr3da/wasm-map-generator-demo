use sdl2::rect::{Rect};
use scenes::game::{Game};
use renderer::renderer::{RenderTarget};
use constants::{tile, renderer};

pub fn toogle_render_target(current_target: RenderTarget) -> RenderTarget {
    match current_target {
        RenderTarget::TopDown => RenderTarget::ThirdPerson,
        RenderTarget::ThirdPerson => RenderTarget::TopDown,
    }
}

pub fn set_initial_position_frame(g: &mut Game) {
    let frame = g.get_map_frame();
    match renderer::INITAL_RENDERER_TARGET {
        RenderTarget::TopDown => set_top_down_person_initial_position(g),
        RenderTarget::ThirdPerson => set_third_person_initial_position(g),
    }
}

fn set_third_person_initial_position(g: &mut Game) {
    let frame = g.get_map_frame();
    let p = Rect::new(0, 0, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT);
    g.set_position_frame(p);
}

fn set_top_down_person_initial_position(g: &mut Game) {
    let frame = g.get_map_frame();
    let p = Rect::new(800, 800, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT);
    g.set_position_frame(p);
}
