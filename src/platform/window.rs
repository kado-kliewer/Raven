use sdl2::render::WindowCanvas;
use sdl2::video::WindowBuilder;
use crate::rendering::Renderer;

pub struct Window {
    renderer: Renderer,
}

impl Window {
    pub fn new(video_subsystem: &sdl2::VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let mut binding = video_subsystem.window(title, width, height);
        let window_builder = binding
            .position_centered();

        let canvas = window_builder.build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let renderer = Renderer::new(canvas);

        Ok(Self {
            renderer,
        })
    }

    pub fn update(&mut self) -> bool {
        // Handle window-specific updates here
        true
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}