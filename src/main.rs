mod chip8;
mod platform;

use std::env;
use std::time::{Duration, Instant};

use chip8::{Chip8, VIDEO_HEIGHT, VIDEO_WIDTH};
use platform::Platform;

use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cargo run <scale> <delay_ms> <rom>");
        std::process::exit(1);
    }

    let video_scale: usize = args[1].parse().expect("Scale must be a number");
    let cycle_delay_ms: u64 = args[2].parse().expect("Delay must be a number");
    let rom_path = &args[3];

    let mut platform = Platform::new(
        "CHIP-8 Emulator",
        VIDEO_WIDTH * video_scale,
        VIDEO_HEIGHT * video_scale,
        VIDEO_WIDTH,
        VIDEO_HEIGHT,
    );

    let mut chip8 = Chip8::new();
    let rom = fs::read(rom_path).expect("Failed to read ROM file");
    chip8.load_rom(&rom);

    let cycle_delay = Duration::from_millis(cycle_delay_ms);
    let mut last_cycle_time = Instant::now();

    loop {
        let quit = platform.process_input(&mut chip8.keypad);

        if quit {
            break;
        }

        let current_time = Instant::now();

        if current_time.duration_since(last_cycle_time) > cycle_delay {
            last_cycle_time = current_time;

            chip8.cycle();

            platform.update(&chip8.screen);
        }
    }
}