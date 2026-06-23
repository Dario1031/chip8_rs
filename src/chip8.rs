use std::fs;

const START_ADDRESS: u16 = 0x200;

pub struct Chip8 {
    registers: [u8; 16],
    pub memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    screen: [u32; 64 * 32],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            screen: [0; 64 * 32],
            opcode: 0,
        }
    }

    pub fn load_rom(&mut self, file_path: &str) {
        let rom = fs::read(file_path).expect("Failed to read ROM file");

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[START_ADDRESS as usize + i] = byte;
        }
    }
}
