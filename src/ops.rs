use crate::{WIDTH, HEIGHT, VF, Chip8};
use crate::screen::{Point, Buffer, Screen};

use rand::{Rng, rngs::ThreadRng};

/// (0nnn - SYS addr)
/// Jump to a machine code routine at nnn.
///
/// This instruction is only used on the old computers on which Chip-8 was originally implemented.
/// It is ignored by modern interpreters.
pub fn sys_jump_to_routine(chip8: &mut Chip8, opcode: u16) {
}

/// (00E0 - CLS)
/// Clear the display.
pub fn cls_clear_display(chip8: &mut Chip8, _opcode: u16) {
    println!("Clear display");
    chip8.display.clear();
}

/// (00EE - RET)
/// Return from a subroutine.
/// 
/// The interpreter sets the program counter to the address at the top of the stack, then
/// subtracts 1 from the stack pointer.
pub fn ret_return_from_subroutine(chip8: &mut Chip8, _opcode: u16) {
    chip8.pc = chip8.stack[chip8.sp as usize];
    chip8.sp -= 1;
}

/// (1nnn - JP addr)
/// Jump to location nnn.
/// 
/// The interpreter sets the program counter to nnn.
pub fn jp_jump_to_address(chip8: &mut Chip8, opcode: u16) {
    chip8.pc = opcode & 0x0FFF;
    println!("Jump to location {:#X?}", chip8.pc);
}

/// (2nnn - CALL addr)
/// Call subroutine at nnn.
/// 
/// The interpreter increments the stack pointer, then puts the current PC on the top of
/// the stack. The PC is then set to nnn.
pub fn call_subroutine(chip8: &mut Chip8, opcode: u16) {
    let subroutine = opcode & 0x0FFF;

    println!("Add pc {:#X?} to stack, run subroutine at {:#X?}",
        chip8.pc, subroutine);

    chip8.sp += 1;
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.pc = subroutine;
}

/// (3xkk - SE Vx, byte)
/// Skip next instruction if Vx = kk.
/// 
/// The interpreter compares register Vx to kk, and if they are equal, increments the program
/// counter by 2.
pub fn se_register_byte(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;
    let kk = decode_byte(opcode);

    let x = chip8.registers[v_x];

    if x == kk {
        chip8.pc += 2;
    }
}

/// (4xkk - SNE Vx, byte)
/// Skip next instruction if Vx != kk.
/// 
/// The interpreter compares register Vx to kk, and if they are not equal, increments
/// the program counter by 2.
pub fn sne_skip_not_equal(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;
    let kk = decode_byte(opcode);

    let x = chip8.registers[v_x];

    if x != kk {
        chip8.pc += 2;
    }
}

/// (5xy0 - SE Vx, Vy)
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal,
/// increments the program counter by 2.
pub fn se_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);
    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];

    if x == y {
        chip8.pc += 2;
    }
}

/// (6xkk - LD Vx, byte)
/// Set Vx = kk.
/// 
/// The interpreter puts the value kk into register Vx.
pub fn ld_register_byte(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;
    let kk = decode_byte(opcode);

    println!("Setting register V{:X?} to {:#X?}", v_x, kk);

    chip8.registers[v_x] = kk;
}

/// (7xkk - ADD Vx, byte)
/// Set Vx = Vx + kk.
/// 
/// Adds the value kk to the value of register Vx, then stores the result in Vx.
pub fn add_register_byte(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;
    let kk = decode_byte(opcode);

    let value = chip8.registers[v_x];

    println!("Adding value {:#X?} to V{:X?} ({:#X?})", kk, v_x, value);

    chip8.registers[v_x] = value.wrapping_add(kk);
}

/// (8xy0 - LD Vx, Vy)
/// Set Vx = Vy.
/// 
/// Stores the value of register Vy in register Vx.
pub fn ld_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);
    let y = chip8.registers[v_y as usize];

    chip8.registers[v_x as usize] = y;
}

/// (8xy1 - OR Vx, Vy)
/// Set Vx = Vx OR Vy.
/// 
/// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
/// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
/// then the same bit in the result is also 1. Otherwise, it is 0.
pub fn or_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);
    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];

    chip8.registers[v_x as usize] = x | y;
}

/// (8xy2 - AND Vx, Vy)
/// Set Vx = Vx AND Vy.
/// 
/// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
/// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
/// then the same bit in the result is also 1. Otherwise, it is 0. 
pub fn and_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];
    
    chip8.registers[v_x as usize] = x & y;
}

/// (8xy3 - XOR Vx, Vy)
/// Set Vx = Vx XOR Vy.
/// 
/// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result
/// in Vx. An exclusive OR compares the corrseponding bits from two values, and if
/// the bits are not both the same, then the corresponding bit in the result is set to 1.
/// Otherwise, it is 0.
pub fn xor_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];
    
    chip8.registers[v_x as usize] = x ^ y;
}

/// (8xy4 - ADD Vx, Vy)
/// Set Vx = Vx + Vy, set VF = carry.
/// 
/// The values of Vx and Vy are added together. If the result is greater than
/// 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits
/// of the result are kept, and stored in Vx.
pub fn add_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];

    let (sum, overflow) = x.overflowing_add(y);

    chip8.registers[v_x as usize] = sum;
    chip8.registers[VF] = overflow as u8;
}

/// (8xy5 - SUB Vx, Vy)
/// Set Vx = Vx - Vy, set VF = NOT borrow.
///
/// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and
/// the results stored in Vx.
pub fn sub_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];

    chip8.registers[v_x as usize] = x - y;
    chip8.registers[VF] = (x > y) as u8;
}

/// (8xy6 - SHR Vx {, Vy})
/// Set Vx = Vx SHR 1.
/// 
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx
/// is divided by 2.
pub fn shr_registers(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;

    let lsb = chip8.registers[v_x] & 0b00000001;
    chip8.registers[VF] = lsb;

    chip8.registers[v_x] = (chip8.registers[v_x] - lsb) / 2
}

/// (8xy7 - SUBN Vx, Vy)
/// Set Vx = Vy - Vx, set VF = NOT borrow.
/// 
/// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and
/// the results stored in Vx.
pub fn subn_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    let x = chip8.registers[v_x as usize];
    let y = chip8.registers[v_y as usize];

    chip8.registers[v_x as usize] = y - x;
    chip8.registers[VF] = (y > x) as u8;
}

/// (8xyE - SHL Vx {, Vy})
/// Set Vx = Vx SHL 1.
/// 
/// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx
/// is multiplied by 2.
pub fn shl_registers(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;

    let msb = chip8.registers[v_x] & 0b00000001;
    chip8.registers[VF] = msb;

    chip8.registers[v_x] = chip8.registers[v_x] * 2
}

/// (9xy0 - SNE Vx, Vy)
/// Skip next instruction if Vx != Vy.
/// 
/// The values of Vx and Vy are compared, and if they are not equal, the program counter
/// is increased by 2.
pub fn sne_registers(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);

    if chip8.registers[v_x as usize] != chip8.registers[v_y as usize] {
        chip8.pc += 2;
    }
}

/// (Annn - LD I, addr)
/// Set I = nnn.
/// 
/// The value of register I is set to nnn.
pub fn ld_i_byte(chip8: &mut Chip8, opcode: u16) {
    chip8.i = opcode & 0x0FFF;

    println!("Set I to {:#X?}", chip8.i);
}

/// (Bnnn - JP V0, addr)
/// Jump to location nnn + V0.
/// 
/// The program counter is set to nnn plus the value of V0.
pub fn jp_bnnn(chip8: &mut Chip8, opcode: u16) {
    let nnn = decode_short(opcode);
    let v0 = (chip8.registers[0]) as u16;

    chip8.pc = nnn + v0;

    println!("Set Program Counter to {:#X?}", chip8.pc);
}

/// (Cxkk - RND Vx, byte)
/// Set Vx = random byte AND kk.
/// 
/// The interpreter generates a random number from 0 to 255, which is then ANDed with
/// the value kk. The results are stored in Vx. See instruction 8xy2 for more information
/// on AND.
pub fn rnd(chip8: &mut Chip8, opcode: u16) {
    let x = decode_register_x(opcode);
    let kk = decode_byte(opcode);

    let random: u8 = chip8.rng.gen();
    println!("Sample {}", random);

    chip8.registers[x as usize] = random & kk;
}

/// (Dxyn - DRW Vx, Vy, n)
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/// 
/// The interpreter reads n bytes from memory, starting at the address stored in I.
/// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
/// Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
/// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it
/// is outside the coordinates of the display, it wraps around to the opposite side of
/// the screen. See instruction 8xy3 for more information on XOR, and section 2.4,
/// Display, for more information on the Chip-8 screen and sprites.
pub fn drw_draw_sprite(chip8: &mut Chip8, opcode: u16) {
    let (v_x, v_y) = decode_registers(opcode);
    let n = (opcode & 0x000F) as u8;

    let x = chip8.registers[v_x as usize] as usize;
    let y = chip8.registers[v_y as usize] as usize;

    let start = chip8.i as usize;
    let end = start + n as usize;

    let read = &chip8.memory[start..end];

    println!("At position ({}, {}), draw:", x, y);
    for byte in read {
        println!("{:08b}", byte);
    }

    /*
    for byte in 0u8..n {
        let idx = (y as usize + byte as usize) * WIDTH + x as usize;
        let bits = binary_to_vec(read[byte as usize]);

        println!("{:08b}, {:?}", byte, bits);

        for (jdx, bit) in bits.iter().enumerate() {
            println!("{}, {}", jdx, bit);
            chip8.display[idx+jdx] = *bit as u32 * 255;
        }
    }
    */
    chip8.display.blit(&binary_to_buffer(read.to_vec()), Point::new(x, y));
}

/// (Ex9E - SKP Vx)
/// Skip next instruction if key with the value of Vx is pressed.
/// 
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently
/// in the down position, PC is increased by 2.
pub fn skp_skip_pressed(chip8: &mut Chip8, opcode: u16) {}

/// (ExA1 - SKNP Vx)
/// Skip next instruction if key with the value of Vx is not pressed.
/// 
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in
/// the up position, PC is increased by 2.
pub fn sknp_skip_not_pressed(chip8: &mut Chip8, opcode: u16) {}

/// (Fx07 - LD Vx, DT)
/// Set Vx = delay timer value.
/// 
/// The value of DT is placed into Vx.
pub fn ld_get_delay_timer(chip8: &mut Chip8, opcode: u16) {
    let v_x = (chip8.opcode & 0x0F00) >> 8;

    chip8.registers[v_x as usize] = chip8.delay_timer;
}

/// (Fx0A - LD Vx, K)
/// Wait for a key press, store the value of the key in Vx.
/// 
/// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn ld_wait_for_key(chip8: &mut Chip8, opcode: u16) {}

/// (Fx15 - LD DT, Vx)
/// Set delay timer = Vx.
/// 
/// DT is set equal to the value of Vx.
pub fn ld_set_delay_timer(chip8: &mut Chip8, opcode: u16) {}

/// (Fx18 - LD ST, Vx)
/// Set sound timer = Vx.
/// 
/// ST is set equal to the value of Vx.
pub fn ld_set_sound_timer(chip8: &mut Chip8, opcode: u16) {}

/// (Fx1E - ADD I, Vx)
/// Set I = I + Vx.
/// 
/// The values of I and Vx are added, and the results are stored in I.
pub fn add_to_i(chip8: &mut Chip8, opcode: u16) {}

/// (Fx29 - LD F, Vx)
/// Set I = location of sprite for digit Vx.
/// 
/// The value of I is set to the location for the hexadecimal sprite corresponding to
/// the value of Vx. See section 2.4, Display, for more information on
/// the Chip-8 hexadecimal font.
pub fn ld_i_to_sprite(chip8: &mut Chip8, opcode: u16) {}

/// (Fx33 - LD B, Vx)
/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
/// 
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in
/// memory at location in I, the tens digit at location I+1, and the ones digit at
/// location I+2.
pub fn ld_bcd(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode) as usize;
    let mut x = chip8.registers[v_x];

    let hundreds = x - x % 100;
    x -= hundreds;

    let tens = x - x % 10;
    let ones = x - tens;

    println!("{}", chip8.registers[v_x]);
    println!("{}, {}, {}", hundreds, tens, ones);
}

/// (Fx55 - LD [I], Vx)
/// Store registers V0 through Vx in memory starting at location I.
/// 
/// The interpreter copies the values of registers V0 through Vx into memory, starting
/// at the address in I.
pub fn ld_store_registers(chip8: &mut Chip8, opcode: u16) {
    let v_x = decode_register_x(opcode);
    let i = chip8.i as usize;

    for (idx, register) in (v_x..16).enumerate() {
        println!("{}, {}", idx, register);

        chip8.memory[i + idx] = chip8.registers[register as usize];
    }
}

/// (Fx65 - LD Vx, [I])
/// Read registers V0 through Vx from memory starting at location I.
/// 
/// The interpreter reads values from memory starting at location I into registers
/// V0 through Vx.
pub fn ld_read_registers(chip8: &mut Chip8, opcode: u16) {

}

fn decode_register_x(opcode: u16) -> u8 {
    let v_x = (opcode & 0x0F00) >> 8;

    v_x as u8
}

fn decode_register_y(opcode: u16) -> u8 {
    let v_y = (opcode & 0x00F0) >> 4;

    v_y as u8
}

fn decode_registers(opcode: u16) -> (u8, u8) {
    (decode_register_x(opcode), decode_register_y(opcode))
}

fn decode_byte(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}

fn decode_short(opcode: u16) -> u16 {
    (opcode & 0x0FFF)
}

fn binary_to_vec(mut binary: u8) -> Vec<u8> {
    let mut values = Vec::new();

    for _ in 0..8 {
        values.push((binary & 0b10000000) >> 7);
        binary = binary << 1;
    }

    return values;
}

fn binary_to_buffer(binary: Vec<u8>) -> Buffer {
    let mut pixels = Vec::new();
    let height = binary.len();

    for bin in binary {
        for pixel in binary_to_vec(bin) {
            pixels.push(pixel as u32 * 255);
        }
    }

    Buffer::new(8, height, Some(pixels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_short() {
        let result = decode_short(0xABCD);
        let expected = 0xBCD;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_binary_to_vec() {
        let result = binary_to_vec(0b00101010);
        let expected = vec!(0, 0, 1, 0, 1, 0, 1, 0);

        assert_eq!(result, expected);
    }
}
