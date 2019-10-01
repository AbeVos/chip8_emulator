extern crate rand;
extern crate minifb;

mod ops;

use std::{
    io, thread, time,
    fs::File,
    io::prelude::*,
};
use rand::{Rng, rngs::ThreadRng};
use minifb::{Key, WindowOptions, Window, Scale};

const MEMORY: usize = 4096;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const VF: usize = 15;

type Register = u8;
type Opcode = u16;

pub struct Chip8 {
    pc: u16,
    opcode: u16,
    i: u16,

    registers: [u8; 16],
    memory: [u8; MEMORY],
    display: [u32; WIDTH * HEIGHT],

    delay_timer: u8,
    sound_timer: u8,

    sp: u16,
    stack: [u16; 16],

    rng: ThreadRng,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            pc: 0x200,
            opcode: 0,
            i: 0,

            registers: [0; 16],
            memory: [0; MEMORY],
            display: [0; WIDTH * HEIGHT],

            delay_timer: 0,
            sound_timer: 0,

            sp: 0,
            stack: [0; 16],

            rng: rand::thread_rng(),
        }
    }

    fn load_rom(&mut self, path: &str) -> io::Result<()> {
        let file = File::open(path)?;

        for (idx, byte) in file.bytes().enumerate() {
            self.memory[idx + 512] = byte.unwrap();
            // println!("Read {:#X?}", self.memory[idx + 512]);
        }

        Ok(())
    }

    fn cycle(&mut self) {
        let pc = self.pc as usize;

        // Fetch opcode
        let opcode_1 = self.memory[pc] as u16;
        let opcode_2 = self.memory[pc + 1] as u16;

        let opcode = opcode_1 << 8 | opcode_2;

        println!("{:#X?} Opcode: {:#X?}", pc, opcode);

        // Decode opcode
        let instruction = self.opcode & 0xF000;

        match instruction {
            0x0000 => {
                match self.opcode & 0x00FF {
                    0x00E0 => ops::cls_clear_display(self, opcode),
                    0x00EE => ops::ret_return_from_subroutine(self, opcode),
                    _ => {},
                }
            },
            0x1000 => ops::jp_jump_to_address(self, opcode),
            0x2000 => ops::call_subroutine(self, opcode),
            0x3000 => ops::se_register_byte(self, opcode),
            0x4000 => ops::sne_skip_not_equal(self, opcode),
            0x5000 => ops::se_registers(self, opcode),
            0x6000 => ops::ld_register_byte(self, opcode),
            0x7000 => ops::add_register_byte(self, opcode),
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => ops::ld_registers(self, opcode),
                    0x0001 => ops::or_registers(self, opcode),
                    0x0002 => ops::and_registers(self, opcode),
                    0x0003 => ops::xor_registers(self, opcode),
                    0x0004 => ops::add_registers(self, opcode),
                    0x0005 => ops::sub_registers(self, opcode),
                    0x0006 => ops::shr_registers(self, opcode),
                    0x0007 => ops::subn_registers(self, opcode),
                    0x000E => ops::shl_registers(self, opcode),
                    _ => {},
                }
            },
            0x9000 => {},
            0xA000 => ops::ld_i_byte(self, opcode),
            0xB000 => ops::jp_bnnn(self, opcode),
            0xC000 => ops::rnd(self, opcode),
            0xD000 => {
                let n_bytes = (self.opcode & 0x000F) as usize;

                let v_x = (self.opcode & 0x0F00) >> 8;
                let v_y = (self.opcode & 0x00F0) >> 4;

                let x = self.registers[v_x as usize];
                let y = self.registers[v_y as usize];

                let start = self.i as usize;
                let end = start + n_bytes;

                let sprite = &self.memory[start..end];

                println!("Draw {:?}, to ({}, {})", sprite, x, y);

                // let draw_start = y as usize * WIDTH + x as usize;
                let draw_start = y as usize + (x - 1) as usize * HEIGHT;

                for (idx, byte) in sprite.iter().enumerate() {
                    self.display[draw_start + idx] = *byte as u32;
                }
            },
            0xE000 => {
                match self.opcode & 0xF0FF {
                    0xE09E => {},
                    0xE0A1 => {},
                    _ => {},
                }
            },
            0xF000 => {
                match self.opcode & 0xF0FF {
                    0xF007 => {
                        let v_x = (self.opcode & 0x0F00) >> 8;

                        self.registers[v_x as usize] = self.delay_timer;
                    },
                    0xF00A => {},
                    0xF015 => {},
                    0xF018 => {},
                    0xF01E => {},
                    0xF029 => {},
                    0xF033 => {
                        let v_x = (self.opcode & 0x0F00) >> 8;
                        let mut x = self.registers[v_x as usize];

                        let hundreds = x - x % 100;
                        x -= hundreds;

                        let tens = x - x % 10;
                        let ones = x - tens;

                        println!("{}", self.registers[v_x as usize]);
                        println!("{}, {}, {}", hundreds, tens, ones);
                    },
                    0xF055 => {},
                    0xF065 => {},
                    _ => {},
                }
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

        println!("");
    }
}

fn main() {
    let mut chip8 = Chip8::new();

    // Load game
    chip8.load_rom("/home/abe/src/chip8_roms/roms/games/Pong (1 player).ch8")
        .expect("Could not open file");

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
        chip8.cycle();

        // Draw graphics
        window.update_with_buffer(&chip8.display).unwrap();

        // Set keys
    }
}
