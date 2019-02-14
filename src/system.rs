use std::sync::{Arc, Mutex};

use sdl2::{render::WindowCanvas, video::WindowBuilder, Sdl, TimerSubsystem, VideoSubsystem};

pub struct System {
    sdl_context: Sdl,
    timer_subsystem: TimerSubsystem,
    video_subsystem: VideoSubsystem,
    canvas: Arc<Mutex<WindowCanvas>>,
}

impl System {
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let timer_subsystem = sdl_context.timer()?;
        let video_subsystem = sdl_context.video()?;

        let window = WindowBuilder::new(&video_subsystem, "pogChamp", 640, 480)
            .build()
            .unwrap();
        let canvas = Arc::new(Mutex::new(window.into_canvas().build().map_err(|s| format!("{}", s))?));

        Ok(System {
            sdl_context,
            timer_subsystem,
            video_subsystem,
            canvas,
        })
    }

    pub fn get_canvas(&self) -> Arc<Mutex<WindowCanvas>> {
        self.canvas.clone()
    }
}
