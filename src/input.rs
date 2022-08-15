extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    pub quit: bool
}

impl Input {
    pub fn set(&mut self, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                _ => {}
            }
        }
    }
}
