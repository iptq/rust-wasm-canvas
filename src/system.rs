use sdl2::{
    video::{Window, WindowBuilder},
    Sdl, TimerSubsystem, VideoSubsystem,
};

pub struct System {
    sdl_context: Sdl,
    timer_subsystem: TimerSubsystem,
    video_subsystem: VideoSubsystem,
    window: Window,
}

impl System {
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let timer_subsystem = sdl_context.timer()?;
        let video_subsystem = sdl_context.video()?;

        let window = WindowBuilder::new(&video_subsystem, "pogChamp", 640, 480)
            .build()
            .unwrap();

        Ok(System {
            sdl_context,
            timer_subsystem,
            video_subsystem,
            window,
        })
    }

    pub fn get_video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }
}
