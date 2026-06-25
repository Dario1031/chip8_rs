use std::fs;
use rand::RngExt;

const START_ADDR: u16 = 0x200;
const FONTSET_START_ADDR: u16 = 0x50;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub keypad: [u8; 16],
    pub screen: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDR,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            screen: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            opcode: 0,
        };

        for (i, &byte) in FONTSET.iter().enumerate() {
            chip8.memory[FONTSET_START_ADDR as usize + i] = byte;
        }

        return chip8;

    }

    pub fn load_rom(&mut self, file_path: &str) {
        let rom = fs::read(file_path).expect("Failed to read ROM file");

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[START_ADDR as usize + i] = byte;
        }
    }

    fn op_00e0(&mut self) {
        self.screen.fill(0);
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn op_1nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = address;
    }

    fn op_2nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn op_3xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] == kk {
            self.pc += 2
        }
    }

    fn op_4xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] != kk {
            self.pc += 2
        }
    }

    fn op_5xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2
        }
    }

    fn  op_6xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = kk;
    }

    fn  op_7xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(kk);
    }

    fn op_8xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    fn op_8xy1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vx as usize] | self.registers[vy as usize];
    }

    fn op_8xy2(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vx as usize] & self.registers[vy as usize];
    }

    fn op_8xy3(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = self.registers[vx as usize] ^ self.registers[vy as usize];
    }

    fn op_8xy4(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        let sum: u16 = 
            self.registers[vx as usize] as u16 + 
            self.registers[vy as usize] as u16;
        if sum > 255 {
            self.registers[0xF as usize] = 1;
        } else {
            self.registers[0xF as usize] = 0;
        }

        self.registers[vx as usize] = (sum & 0x00FF) as u8;
    }

    fn op_8xy5(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] > self.registers[vy as usize] {
            self.registers[0xF as usize] = 1;
        } else {
            self.registers[0xF as usize] = 0;
        }

        // wrapping sub in case of subtraction underflow
        self.registers[vx as usize] =
            self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);
    }

    fn op_8xy6(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF as usize] = self.registers[vx as usize] & 0x1;
        // divide by 2 == right shift 1
        self.registers[vx as usize] = self.registers[vx as usize] >> 1;
    }

    fn op_8xy7(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] < self.registers[vy as usize] {
            self.registers[0xF as usize] = 1;
        } else {
            self.registers[0xF as usize] = 0;
        }

        // wrapping sub in case of subtraction underflow
        self.registers[vx as usize] =
            self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);
    }

    fn op_8xye(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF as usize] = (self.registers[vx as usize] & 0x80) >> 7;
        // multiply by 2 == left shift 1
        self.registers[vx as usize] = self.registers[vx as usize] << 1;
    }

    fn op_9xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.index = address;
    }

    fn op_bnnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = self.registers[0] as u16 + address;
    }

    fn op_cxkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let kk: u8 = (self.opcode & 0x00FF) as u8;

        let mut rng = rand::rng();
        let random_byte: u8 = rng.random();

        self.registers[vx as usize] = random_byte & kk;
    }

    fn op_dxyn(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let n: u8 = (self.opcode & 0x000F) as u8;

        let x_pos = self.registers[vx as usize] as usize % VIDEO_WIDTH;
        let y_pos = self.registers[vy as usize] as usize % VIDEO_HEIGHT;

        self.registers[0xF as usize] = 0;

        for i in 0..n {
            let sprite_byte: u8 = self.memory[self.index as usize + i as usize];
            // sprites guaranteed to be 8px wide -> 8 columns
            for col in 0..8 {
                let sprite_pixel: u8 = sprite_byte & (0x80 >> col);
                let y = (y_pos as usize + i as usize) % VIDEO_HEIGHT;
                let x = (x_pos as usize + col as usize) % VIDEO_WIDTH;
                let screen_index = y * VIDEO_WIDTH + x;
                let screen_pixel: u32 = self.screen[screen_index];
                if sprite_pixel != 0 {
                    if screen_pixel == 0xFFFFFFFF {
                        self.registers[0xF as usize] = 1;
                    }

                    self.screen[screen_index] = screen_pixel ^ 0xFFFFFFFF;
                }
            }
        }
    }

    fn op_ex9e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] == 1 {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.registers[vx as usize] = self.delay_timer;
    }

    fn op_fx0a(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for key in 0..16 {
            if self.keypad[key as usize] == 1 {
                self.registers[vx as usize] = key as u8;
                return;
            }
        }

        // wait until a key is pressed
        self.pc -= 2;
    }

    fn op_fx15(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.delay_timer = self.registers[vx as usize];
    }

    fn op_fx18(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.sound_timer = self.registers[vx as usize];
    }

    fn op_fx1e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.index += self.registers[vx as usize] as u16;
    }

    fn op_fx29(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit: u8 = self.registers[vx as usize];

        self.index = FONTSET_START_ADDR + (5 * digit) as u16;
    }

    fn op_fx33(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let mut value: u8 = self.registers[vx as usize];

        // ones digit 
        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;

        // tens digit
        self.memory[self.index as usize + 1] = value % 10;
        value /= 10;

        // hundreds digit
        self.memory[self.index as usize] = value % 10;
    }

    fn op_fx55(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for i in 0..vx {
            self.memory[self.index as usize + i as usize] = self.registers[i as usize];
        }
    }

    fn op_fx65(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for i in 0..vx {
            self.registers[i as usize] = self.memory[self.index as usize + i as usize];
        }
    }

    fn op_null(&mut self) {
        panic!("Unknown opcode: {:04X}", self.opcode);
    }

    pub fn decode_and_execute_opcode(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => self.op_00e0(),
                0x000E => self.op_00ee(),
                _ => self.op_null(),
            },

            0x1000 => self.op_1nnn(),
            0x2000 => self.op_2nnn(),
            0x3000 => self.op_3xkk(),
            0x4000 => self.op_4xkk(),
            0x5000 => self.op_5xy0(),
            0x6000 => self.op_6xkk(),
            0x7000 => self.op_7xkk(),

            0x8000 => match self.opcode & 0x000F {
                0x0000 => self.op_8xy0(),
                0x0001 => self.op_8xy1(),
                0x0002 => self.op_8xy2(),
                0x0003 => self.op_8xy3(),
                0x0004 => self.op_8xy4(),
                0x0005 => self.op_8xy5(),
                0x0006 => self.op_8xy6(),
                0x0007 => self.op_8xy7(),
                0x000E => self.op_8xye(),
                _ => self.op_null(),
            },

            0x9000 => self.op_9xy0(),
            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxkk(),
            0xD000 => self.op_dxyn(),

            0xE000 => match self.opcode & 0x00FF {
                0x009E => self.op_ex9e(),
                0x00A1 => self.op_exa1(),
                _ => self.op_null(),
            },

            0xF000 => match self.opcode & 0x00FF {
                0x0007 => self.op_fx07(),
                0x000A => self.op_fx0a(),
                0x0015 => self.op_fx15(),
                0x0018 => self.op_fx18(),
                0x001E => self.op_fx1e(),
                0x0029 => self.op_fx29(),
                0x0033 => self.op_fx33(),
                0x0055 => self.op_fx55(),
                0x0065 => self.op_fx65(),
                _ => self.op_null(),
            },

            _ => self.op_null(),
        }
    }

    pub fn cycle(&mut self) {
        // fetch
        let opcode: u16 = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[self.pc as usize + 1] as u16;

        self.pc += 2;

        // decode and execute
        self.opcode = opcode;
        self.decode_and_execute_opcode();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

}
