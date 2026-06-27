#![cfg(not(target_arch = "wasm32"))]

use minifb::{Key, Window, WindowOptions};
use crate::chip8::Chip8;

pub struct Platform {
    window: Window,
    width: usize,
    height: usize,
}

impl Platform {
    pub fn new(title: &str, window_width: usize, window_height: usize, texture_width: usize, texture_height: usize,) -> Self {
        let window = Window::new(title, window_width, window_height, WindowOptions::default(),).expect("Failed to create window");

        Self {window, width: texture_width, height: texture_height,}
    }

    pub fn update(&mut self, buffer: &[u32]) {
        self.window.update_with_buffer(buffer, self.width, self.height).expect("Failed to update window buffer");
    }

    pub fn process_input(&mut self, chip8: &mut Chip8) -> bool {
        let mappings = [
            (Key::X, 0x0),
            (Key::Key1, 0x1),
            (Key::Key2, 0x2),
            (Key::Key3, 0x3),
            (Key::Q, 0x4),
            (Key::W, 0x5),
            (Key::E, 0x6),
            (Key::A, 0x7),
            (Key::S, 0x8),
            (Key::D, 0x9),
            (Key::Z, 0xA),
            (Key::C, 0xB),
            (Key::Key4, 0xC),
            (Key::R, 0xD),
            (Key::F, 0xE),
            (Key::V, 0xF),
        ];

        for (key, chip8_key) in mappings {
            if self.window.is_key_down(key) {
                chip8.key_down(chip8_key);
            } else {
                chip8.key_up(chip8_key);
            }
        }

        !self.window.is_open() || self.window.is_key_down(Key::Escape)
    }
}