use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self { canvas }
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn draw_rectangle(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).expect("Failed to draw rectangle");
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: Color) {
        self.canvas.set_draw_color(color);
        for dy in -radius..=radius {
            let dx = (radius * radius - dy * dy) as f32;
            let dx = dx.sqrt() as i32;
            self.canvas.draw_line((x - dx, y + dy), (x + dx, y + dy))
                .expect("Failed to draw circle line");
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}