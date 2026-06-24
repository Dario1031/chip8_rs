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

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
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
            screen: [0; 64 * 32],
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

        self.registers[vx as usize] += kk;
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

}
