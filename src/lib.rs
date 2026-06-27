mod chip8;

use crate::chip8::Chip8;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmChip8 {
    chip8: Chip8,
}

#[wasm_bindgen]
impl WasmChip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmChip8 {
        WasmChip8 {
            chip8: Chip8::new(),
        }
    }

    pub fn cycle(&mut self) {
        self.chip8.cycle();
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.chip8.load_rom(&rom);
    }

    pub fn key_down(&mut self, key: usize) {
        self.chip8.key_down(key);
    }

    pub fn key_up(&mut self, key: usize) {
        self.chip8.key_up(key);
    }

    pub fn framebuffer_ptr(&self) -> *const u32 {
        self.chip8.framebuffer().as_ptr()
    }

    pub fn width(&self) -> usize {
        crate::chip8::VIDEO_WIDTH
    }

    pub fn height(&self) -> usize {
        crate::chip8::VIDEO_HEIGHT
    }
}