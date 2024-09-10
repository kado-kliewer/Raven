mod platform;
mod rendering;

use platform::Platform;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::event;

fn main() -> Result<(), String> {
    let mut platform = Platform::new("Game Window", 800, 600)?;

    let mut is_running = true;

    while is_running {
        is_running = platform.update();

        let renderer = platform.window().renderer();
        renderer.clear(Color::RGB(0,0,0));

        renderer.draw_rectangle(Rect::new(300, 200, 200, 200), Color::RGB(255,255,255));

        renderer.draw_circle(300,200,100, Color::RGB(255,0,0));

        renderer.present();

        // Cap the frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
