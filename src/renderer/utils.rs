use sdl2::rect::{Rect};
use scenes::game::{Game};
use renderer::renderer::{RenderTarget};
use constants::{renderer, map};

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
    let (mw, mh) = g.get_map_size();
    let (px, py) = g.get_player_position();
    let p = Rect::new(px, py, mw, mh);
    g.set_scroll_frame(p);
}

fn set_top_down_person_initial_scroll(g: &mut Game) {
    let (mw, mh) = g.get_map_size();
    let (mut px, mut py) = g.get_player_position();
    
    if px == 0 {
        px = map::IGNORE_BORDER_PIXELS + 1; 
    }

    if py == 0 {
        py = map::IGNORE_BORDER_PIXELS + 1;
    }
    
    g.set_scroll_frame(Rect::new(px, py, mw, mh));
}
