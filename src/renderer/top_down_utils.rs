use sdl2::render::{Canvas, RenderTarget, BlendMode, TextureCreator, Texture};
use sdl2::rect::{Rect};
use sdl2::video::{WindowContext};
use light::circle::{Circle};
use light::intensity::{calculate_light_intensity};
use scenes::game::{Game};
use scenes::layer::{Renderable};
use constants::{window, tile, player};
use entities::entity::{Entity};
use utils;

pub fn create_map_texture<'l, T>(t_creator: &'l TextureCreator<WindowContext>, game: &Game, canvas: &mut Canvas<T>) -> Texture<'l> where T: RenderTarget {
    let mut frame = game.get_map_frame();

    let width = frame.width();
    frame.set_width(width * tile::WIDTH);

    let height = frame.height();
    frame.set_height(height * tile::HEIGHT);

    let mut map = utils::texture::create_texture_target(&t_creator, frame.width(), frame.height()).unwrap();
    canvas.with_texture_canvas(&mut map, |texture| {
        for (_, layer) in game.get_map_layers().into_iter(){
            for e in layer.get_entities() {
                let f = e.get_frame();
                texture.set_draw_color(e.get_background());
                texture.fill_rect(Rect::new(f.x() * tile::WIDTH as i32, f.y() * tile::WIDTH as i32, tile::WIDTH as u32, tile::HEIGHT as u32)).unwrap();
            }
        }
    }).unwrap();
    map
}

pub fn create_light_map_texture<'l, T>(t_creator: &'l TextureCreator<WindowContext>, game: &Game, canvas: &mut Canvas<T>) -> Texture<'l> where T: RenderTarget {
    let mut light_map = utils::texture::create_texture_target(&t_creator, window::WIDTH, window::HEIGHT).unwrap();
    canvas.with_texture_canvas(&mut light_map, |texture| render_light(&game, texture)).unwrap();
    light_map.set_blend_mode(BlendMode::Blend);
    light_map
}

pub fn render_map<T>(game: &Game, canvas: &mut Canvas<T>, px: i32, py: i32) where T: RenderTarget {
    let (columns, _) = game.get_map_size();
    let viewport = get_viewport(px, py);

    for (_, layer) in game.get_map_layers().into_iter(){
        let entities = layer.get_entities();
        render_layer_in_viewport(viewport, columns as i32, &mut |index: usize| {
            let e = &entities[index];
            canvas.set_draw_color(e.get_background());
            canvas.fill_rect(e.get_frame()).unwrap();
        });
    }
}

pub fn render_light<T>(_game: &Game, canvas: &mut Canvas<T>) where T: RenderTarget {
    let (px, py) = (0,0); // Todo need to align with player positions
    let light_radius = Circle::new(px + (window::WIDTH as f32 * 0.5) as i32, py + (window::HEIGHT as f32 * 0.5) as i32, player::LIGHT_RADIUS);
    let viewport = Rect::new(0, 0, window::WIDTH / tile::WIDTH + tile::WIDTH, window::HEIGHT / tile::HEIGHT + tile::HEIGHT);

    for y in 0..viewport.height() {
        for x in 0..viewport.width() {
            let f = Rect::new(x as i32 * tile::WIDTH as i32, y as i32 * tile::HEIGHT as i32, tile::WIDTH, tile::HEIGHT);
            canvas.set_draw_color(calculate_light_intensity(&f, &light_radius));
            canvas.fill_rect(f).unwrap();
        }
    }
}

fn render_layer_in_viewport(viewport: Rect, columns: i32, render_cb: &mut FnMut(usize)) {
    for (i, x) in (viewport.x() .. viewport.width() as i32 + 1).enumerate() {
        render_cb(x as usize);
        for y in viewport.y() .. viewport.height() as i32 + 1 {
            render_cb((i as i32 + y * columns as i32) as usize);
        }
    }
}

fn get_viewport(x: i32, y: i32) -> Rect {
    Rect::new(x / tile::WIDTH as i32, y / tile::HEIGHT as i32, window::WIDTH / tile::WIDTH, window::HEIGHT/ tile::HEIGHT)
}
