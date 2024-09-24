/*
    Handles all opcode instructions.
    First the two byte instruction is split into nibbles.
    The resulting nibbles are then matched to the corresponding instruction
    which is then executed.
    A copy of the instruction set can be found here: https://johnearnest.github.io/Octo/docs/chip8ref.pdf
*/

use crate::emulation;

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
                emu.program_counter += emulation::INSTRUCTION_SIZE;
            }
        },

        // SKIP VX != NN - if register x != NN skip to the next line (ifelse)
        (4, _, _, _) => {
            let x = nibble2 as usize;
            let nn = (op & 0xFF) as u8;
            if emu.registers[x] != nn {
                emu.program_counter += emulation::INSTRUCTION_SIZE;
            }
        },

        // SKIP VX == VY - check if register x == register y
        (5, _, _, 0) => {
            let x = nibble2 as usize;
            let y = nibble3 as usize;
            if emu.registers[x] == emu.registers[y] {
                emu.program_counter += emulation::INSTRUCTION_SIZE;
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

        // else
        (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
    }
}