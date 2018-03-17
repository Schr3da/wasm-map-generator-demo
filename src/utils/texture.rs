use sdl2::render::{TextureValueError};
use sdl2::pixels::{PixelFormatEnum};
use sdl2::render::{Texture};
use sdl2::render::{TextureCreator};
use sdl2::video::{WindowContext};

pub fn create_texture_target(t_creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> Result<Texture, TextureValueError> {
    t_creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)
}
