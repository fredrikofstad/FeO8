use crate::font;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const REGISTER_NUM: usize = 16;
const NUM_KEYS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Emulation {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    registers: [u8; REGISTER_NUM],
    i_register: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

// The contents of rom are copied to address 512 in ram
const START_ADDRESS: u16 = 0x200;

impl Emulation {
    pub fn new() -> Self {
        let mut new_emulation = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            registers: [0; REGISTER_NUM],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        // loads the fontset data into unused ram space (before rom data)
        new_emulation.ram[..font::SET_SIZE].copy_from_slice(&font::SET);
        new_emulation
    }

    // resets the emulation to default values
    pub fn reset(&mut self) {
        self.program_counter = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.registers = [0; REGISTER_NUM];
        self.i_register = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..font::SET_SIZE].copy_from_slice(&font::SET);
    }

    // pushes and popping values to and from the stack
    fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }
    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    // CPU instructions
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        self.execute(op);
    }

    // fetches the next cpu instruction for execution
    // all instructions are 2 bytes -> returns 16-bit opcode combined as Big Endian
    fn fetch(&mut self) -> u16 {
        let first_byte = self.ram[self.program_counter as usize] as u16;
        let second_byte = self.ram[(self.program_counter + 1) as usize] as u16;
        let op = (first_byte << 8) | second_byte;
        self.program_counter += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let hex1 = (op & 0xF000) >> 12;
        let hex2 = (op & 0x0F00) >> 8;
        let hex3 = (op & 0x00F0) >> 4;
        let hex4 = op & 0x000F;

        match (hex1, hex2, hex3, hex4) {
            // NOP
            (0, 0, 0, 0) => return,
            // else
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }


    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        // produces a beep when 0 is reached
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: implement sound output
            }
            self.sound_timer -= 1;
        }
    }

}