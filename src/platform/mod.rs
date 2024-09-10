use sdl2::{EventPump, Sdl};

mod window;
mod input;
mod file_system;

use file_system::FileSystem;
use input::Input;
use window::Window;

pub struct Platform {
    sdl_context: Sdl,
    event_pump: EventPump,
    window: Window,
    input: Input,
    file_system: FileSystem,
}

impl Platform {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        let window = Window::new(&video_subsystem, title, width, height)?;

        Ok(Self {
            sdl_context,
            event_pump,
            window,
            input: Input::new(),
            file_system: FileSystem::new(),
        })
    }

    pub fn update(&mut self) -> bool {
        self.input.update(&mut self.event_pump);
        self.window.update()
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }
}