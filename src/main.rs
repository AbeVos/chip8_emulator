extern crate bit_vec;

use bit_vec::BitVec;

type Register = u8;
type Opcode = u16;

struct Chip8 {
    registers: [u8; 16],
    memory: BitVec,
    I: u16,
    pc: u16,
    display: BitVec,

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,
}

impl Chip8 {
    fn new() -> Chip8 {
        // TODO: Initialize registers and memory.
        Chip8 {
            pc: 0x200,
        }
    }

    fn cycle(&mut self) {
        // Fetch opcode
        // Decode opcode
        // Execute opcode
        // Update timers
    }
}

fn main() {
    let mut chip8 = Chip8::new();

    // Load game

    loop {
        chip8.cycle();

        // Draw graphics
        // Set keys
    }
}
