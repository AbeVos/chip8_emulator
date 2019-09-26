extern crate bit_vec;
extern crate minifb;

use bit_vec::BitVec;
use minifb::{Key, WindowOptions, Window, Scale};

const MEMORY: usize = 4096;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

type Register = u8;
type Opcode = u16;

struct Chip8 {
    pc: u16,
    opcode: u16,
    i: u16,
    sp: u16,

    // registers: [u8; 16],
    memory: [u16; MEMORY],
    display: [u32; WIDTH * HEIGHT],

    delay_timer: u8,
    sound_timer: u8,

    // stack: [u16; 16],
    // stack_pointer: u16,
}

impl Chip8 {
    fn new() -> Chip8 {
        // TODO: Initialize registers and memory.
        //
        Chip8 {
            pc: 0x200,
            opcode: 0,
            i: 0,
            sp: 0,

            //memory: BitVec::with_capacity(MEMORY),
            memory: [0; MEMORY],
            display: [0; WIDTH * HEIGHT],

            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn load_rom(&mut self, path: &str) {
        /*
        for i = 0..buffer_size {
            self.memory[i] = buffer[i]
        }
         */
    }

    fn cycle(&mut self) {
        let pc = self.pc as usize;

        // Fetch opcode
        self.opcode = self.memory[pc] << 8 | self.memory[pc + 1];

        // Decode opcode
        let instruction = self.opcode & 0xF000;

        match instruction {
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
            },
            _ => {
                println!("Opcode {} not implemented", self.opcode);
            },
        };

        self.pc += 2;

        // Execute opcode
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 1 {
                println!("BEEP");
            }
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();

    // Load game

    // Prepare frame buffer
    let mut window = Window::new(
        "CHIP-8 - ESC to exit",
        WIDTH, HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X4,
            ..WindowOptions::default()
        })
        .unwrap_or_else(|e| { panic!("{}", e); });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // chip8.cycle();

        // Draw graphics
        window.update_with_buffer(&chip8.display).unwrap();
        // Set keys
    }
}
