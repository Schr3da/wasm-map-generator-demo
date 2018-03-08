#[cfg(target_os = "emscripten")]
use emscripten::{emscripten};

use input::keyboard::{KeyboardControls};

pub fn update(keyboard_controlls: &mut KeyboardControls) -> bool {
    let mut should_update = false;

    keyboard_controlls.update();
    for (_key_code, value) in keyboard_controlls.get_keys().into_iter(){
        if value == &true {
            should_update = true;
            break;
        }
    }

    should_update
}

pub fn renderer<F>(mut cb: F) where F: FnMut(bool) -> bool {
    let mut is_init_done = false;

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(|| {
        let _ = cb(is_init_done);
        is_init_done = true;
    });

    #[cfg(not(target_os = "emscripten"))]
    'apploop: loop {
        if !cb(is_init_done) {
            break 'apploop;
        }
        is_init_done = true;
    }
}
