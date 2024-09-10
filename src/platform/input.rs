use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::collections::HashSet;

pub struct Input {
    keys_pressed: HashSet<Keycode>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_position: (i32, i32),
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_position: (0,0),
        }
    }

    pub fn update(&mut self, event_pump: &mut sdl2::EventPump) {
        self.keys_pressed.clear();
        self.mouse_buttons_pressed.clear();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    self.keys_pressed.insert(keycode);
                },
                Event::MouseButtonDown {mouse_btn, ..} => {
                    self.mouse_buttons_pressed.insert(mouse_btn);
                },
                Event::MouseMotion {x, y, ..} => {
                    self.mouse_position = (x, y);
                },
                _ => {}
            }
        }
    }


    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.keys_pressed.contains(&keycode)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }
}