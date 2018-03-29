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

pub fn set_initial_scroll(g: &mut Game) {
    match renderer::INITAL_RENDERER_TARGET {
        RenderTarget::TopDown => set_top_down_person_initial_scroll(g),
        RenderTarget::ThirdPerson => set_third_person_initial_scroll(g),
    }
}

fn set_third_person_initial_scroll(g: &mut Game) {
    let frame = g.get_map_frame();
    let p = Rect::new(0, 0, frame.width() * tile::WIDTH, frame.height() * tile::HEIGHT);
    g.set_scroll_frame(p);
}

fn set_top_down_person_initial_scroll(g: &mut Game) {
    let frame = g.get_map_frame();
    let px = (frame.width() as f32 * 0.5) as i32;
    let py = (frame.height() as f32 * 0.5) as i32;
    g.set_scroll_frame(Rect::new(px, py, frame.width(), frame.height()));
}
