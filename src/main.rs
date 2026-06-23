mod chip8;

use std::env;
use chip8::Chip8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: chip8 <rom>");
        std::process::exit(1);
    }

    let mut chip8 = Chip8::new();
    chip8.load_rom(&args[1]);

    for i in 0x200..0x220 {
        println!("{:03X}: {:02X}", i, chip8.memory[i]);
    }
}
