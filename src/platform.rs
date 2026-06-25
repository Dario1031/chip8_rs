use minifb::{Key, Window, WindowOptions};

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

    pub fn process_input(&mut self, keys: &mut [u8; 16]) -> bool {
        keys[0x0] = self.window.is_key_down(Key::X) as u8;
        keys[0x1] = self.window.is_key_down(Key::Key1) as u8;
        keys[0x2] = self.window.is_key_down(Key::Key2) as u8;
        keys[0x3] = self.window.is_key_down(Key::Key3) as u8;
        keys[0x4] = self.window.is_key_down(Key::Q) as u8;
        keys[0x5] = self.window.is_key_down(Key::W) as u8;
        keys[0x6] = self.window.is_key_down(Key::E) as u8;
        keys[0x7] = self.window.is_key_down(Key::A) as u8;
        keys[0x8] = self.window.is_key_down(Key::S) as u8;
        keys[0x9] = self.window.is_key_down(Key::D) as u8;
        keys[0xA] = self.window.is_key_down(Key::Z) as u8;
        keys[0xB] = self.window.is_key_down(Key::C) as u8;
        keys[0xC] = self.window.is_key_down(Key::Key4) as u8;
        keys[0xD] = self.window.is_key_down(Key::R) as u8;
        keys[0xE] = self.window.is_key_down(Key::F) as u8;
        keys[0xF] = self.window.is_key_down(Key::V) as u8;

        !self.window.is_open() || self.window.is_key_down(Key::Escape)
    }
}