/*
    Handles all opcode instructions.
    First the two byte instruction is split into nibbles.
    The resulting nibbles are then matched to the corresponding instruction
    which is then executed.
    A copy of the instruction set can be found here: https://johnearnest.github.io/Octo/docs/chip8ref.pdf
*/
use rand::random;

use crate::{emulation, sprites};

pub fn execute(emu: &mut emulation::Emulation, op: u16) {
    let nibble1 = (op & 0xF000) >> 12;
    let nibble2 = (op & 0x0F00) >> 8;
    let nibble3 = (op & 0x00F0) >> 4;
    let nibble4 = op & 0x000F;

    match (nibble1, nibble2, nibble3, nibble4) {
        // NOP
        (0, 0, 0, 0) => return,
        // CLS - Clear Screen
        (0, 0, 0xE, 0) => {
            emu.frame_buffer = [false; emulation::SCREEN_WIDTH * emulation::SCREEN_HEIGHT];
        },
        // RET - Return from Subroutine to the address stored on stack
        (0, 0, 0xE, 0xE) => {
            let ret_addr = emu.pop();
            emu.program_counter = ret_addr;
        },
        // JMP NNN - jump to a given address
        (1, _, _, _) => {
            let nnn = op & 0xFFF;
            emu.program_counter = nnn;
        },
        // CALL NNN - push current address to stack and go to subroutine address
        (2, _, _, _) => {
            let nnn = op & 0xFFF;
            emu.push(emu.program_counter);
            emu.program_counter = nnn;
        },
        // SKIP VX == NN - if register x == NN skip to the next line (ifelse)
        (3, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            if emu.registers[x] == nn {
                emu.next_instruction();
            }
        },

        // SKIP VX != NN - if register x != NN skip to the next line (ifelse)
        (4, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            if emu.registers[x] != nn {
                emu.next_instruction();
            }
        },

        // SKIP VX == VY - check if register x == register y
        (5, _, _, 0) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            if emu.registers[x] == emu.registers[y] {
                emu.next_instruction();
            }
        },

        // VX = NN - sets register X to value NN
        (6, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            emu.registers[x] = nn;
        },

        // VX += NN - adds NN to register X, no carry flag, no overflow
        (7, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            emu.registers[x] = emu.registers[x].wrapping_add(nn);
        },

        // VX = VY - sets register x's value to register y's value
        (8, _, _, 0) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            emu.registers[x] = emu.registers[y]
        },

        //  - Bitwise operators between register X and Y -
        // VX |= VY
        (8, _, _, 1) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            emu.registers[x] |= emu.registers[y];
        },

        // VX &= VY
        (8, _, _, 2) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            emu.registers[x] &= emu.registers[y];
        },
        // VX ^= VY
        (8, _, _, 3) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            emu.registers[x] ^= emu.registers[y];
        },

        // VX += VY - adds y's value to x, and sets carry flag in case of overflow
        (8, _, _, 4) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            let (x_value, carry) = emu.registers[x].overflowing_add(emu.registers[y]);
            emu.registers[x] = x_value;
            emu.registers[0xF] = if carry { 1 } else { 0 }; // carry flag register
        },

        // VX -= VY - same as above but with subtraction, sets flag to 0 if borrowing
        (8, _, _, 5) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            let (x_value, borrow) = emu.registers[x].overflowing_sub(emu.registers[y]);
            emu.registers[x] = x_value;
            emu.registers[0xF] = if borrow { 0 } else { 1 };
        },

        // VX >>= 1, right bit shift on register x's value, dropped bit stored in f register
        (8, _, _, 6) => {
            let x = nibble2 as usize;
            let lsb = emu.registers[x] & 1;
            emu.registers[x] >>= 1;
            emu.registers[0xF] = lsb;
        },

        // VX = VY - VX - subtracts x register from y register, saving borrow in f register
        (8, _, _, 7) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            let (x_value, borrow) = emu.registers[y].overflowing_sub(emu.registers[x]);
            emu.registers[x] = x_value;
            emu.registers[0xF] = if borrow { 0 } else { 1 };
        },

        // VX <<= 1, bit shift left storing overflow bit into f register
        (8, _, _, 0xE) => {
            let x = nibble2 as usize;
            let msb = (emu.registers[x] >> 7) & 1;
            emu.registers[x] <<= 1;
            emu.registers[0xF] = msb;
        },

        // SKIP VX != VY - skips the next line if x register value is not the same as y register
        (9, _, _, 0) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            if emu.registers[x] != emu.registers[y] {
                emu.next_instruction();
            }
        },
        // I = NNN - sets the i register (pointer to ram address)
        (0xA, _, _, _) => {
            let nnn = op & 0xFFF;
            emu.i_register = nnn;
        },

        // JMP V0 + NNN - jumps to the sum of register 0 and NNN
        (0xB, _, _, _) => {
            let nnn = op & 0xFFF;
            emu.program_counter = (emu.registers[0] as u16) + nnn;
        },

        // VX = rand() & NN - RNG generator, takes a random u8 and ands with NN
        (0xC, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            let rng: u8 = random();
            emu.registers[x] = rng & nn;
        },

        (0xD, _, _, _) => {
            sprites::draw_sprite(emu, nibble2, nibble3, nibble4);
        }

        // User input

        // SKIP KEY PRESS - skips the next instruction if the key in register x is pressed
        (0xE, _, 9, 0xE) => {
            let x = nibble2 as usize;
            let x_value = emu.registers[x];
            let key = emu.keys[x_value as usize];
            if key {
                emu.next_instruction();
            }
        },



        // else
        (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
    }
}