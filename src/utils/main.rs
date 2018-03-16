#[cfg(target_os = "emscripten")]
use emscripten::{emscripten};

pub fn game_loop<F>(mut cb: F) where F: FnMut() -> bool {

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(|| {
        let _ = cb();
    });

    #[cfg(not(target_os = "emscripten"))]
    'apploop: loop {
        if !cb() {
            break 'apploop;
        }
    }
}
