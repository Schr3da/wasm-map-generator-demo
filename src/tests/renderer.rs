use sdl2;
use std::time::Instant;
use std::process::{exit};
use scenes::game::{Game};
use utils::canvas;
use renderer::{top_down_utils};

#[test]
fn map_and_light_texture() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = canvas::get_canvas(sdl_context);
    let texture_creator = canvas.texture_creator();
    let game = Game::new();

    println!("Start {:?}\n", Instant::now());
    
    top_down_utils::create_map_texture(&texture_creator, &game, &mut canvas);
    top_down_utils::create_light_map_texture(&texture_creator, &game, &mut canvas);
    
    println!("End {:?}\n\n", Instant::now());
}


