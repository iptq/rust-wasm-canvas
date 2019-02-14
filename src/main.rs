mod system;

use std::os::raw;

use failure::Error;
use sdl2::pixels::Color;
use ref_thread_local::{ref_thread_local, RefThreadLocal};

use crate::system::System;

type EmCallbackFunc = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    fn emscripten_set_main_loop(_: EmCallbackFunc, _: raw::c_int, _: raw::c_int);
}

ref_thread_local! {
    static managed global_system: System = System::init().unwrap();
}

extern "C" fn main_loop() {
    let system = global_system.borrow();
    let canvas = system.get_canvas();
    let mut canvas = canvas.lock().unwrap();

    canvas.set_draw_color(Color::RGB(0, 100, 200));
    canvas.clear();
}

fn main() -> Result<(), Error> {
    unsafe {
        use sdl2::sys::*;
        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 2);
        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 0);
        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_DOUBLEBUFFER, 1);
        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_DEPTH_SIZE, 24);
    }

    unsafe {
        emscripten_set_main_loop(Some(main_loop), 0, 1);
    }

    Ok(())
}
