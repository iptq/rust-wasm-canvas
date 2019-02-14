use sdl2::{Sdl, TimerSubsystem, VideoSubsystem};

pub struct System {
    sdl_context: Sdl,
    timer_subsystem: TimerSubsystem,
    video_subsystem: VideoSubsystem,
}

impl System {
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let timer_subsystem = sdl_context.timer()?;
        let video_subsystem = sdl_context.video()?;
        Ok(System {
            sdl_context,
            timer_subsystem,
            video_subsystem,
        })
    }
}
