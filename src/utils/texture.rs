use sdl2::pixels::{Color};
use sdl2::render::{TextureQuery, TextureValueError};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::rect::{Rect};
use sdl2::render::{Texture};
use sdl2::render::{TextureCreator};
use sdl2::video::{WindowContext};
use constants::{window};
use loader::font::{get_centered_rect, get_font_for_key};
use loader::assets::{FontManager};
use entities::entity::{Entity};
use entities::unit::{Unit};

pub fn get_texture<'l>(f_manager: &mut FontManager, t_creator: &'l TextureCreator<WindowContext>) -> (Texture<'l>, Rect) {

    let y = Unit::new(Rect::new(0, 0, 20, 20), Color::RGBA(255, 0, 0, 255), Color::RGBA(0, 255, 0, 255), "s", 10);
    let font = get_font_for_key(f_manager, "normal");
    let surface = font.render(y.get_sign()).blended(y.get_foreground()).unwrap();
    let texture = t_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let padding = 10;
    let target = get_centered_rect(width, height, window::WIDTH - padding, window::HEIGHT -  padding);

    return (texture, target)
}

pub fn create_texture_target(t_creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> Result<Texture, TextureValueError> {
    t_creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)
}
