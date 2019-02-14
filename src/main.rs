mod system;

use std::os::raw;
use std::sync::{Arc, Mutex};

use failure::Error;
use ref_thread_local::{ref_thread_local, RefThreadLocal};
use sdl2::video::WindowBuilder;

use crate::system::System;

type EmCallbackFunc = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    fn emscripten_set_main_loop(_: EmCallbackFunc, _: raw::c_int, _: raw::c_int);
}

ref_thread_local! {
    static managed global_system: Arc<Mutex<System>> = Arc::new(Mutex::new(System::init().unwrap()));
}

extern "C" fn main_loop() {
    let system = global_system.borrow();
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
