use crate::font;
use crate::cpu;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const INSTRUCTION_SIZE: u16 = 2;

const RAM_SIZE: usize = 4096;
const REGISTER_NUM: usize = 16;
const NUM_KEYS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Emulation {
    pub(crate) program_counter: u16,
    pub(crate) ram: [u8; RAM_SIZE],
    pub(crate) frame_buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub(crate) registers: [u8; REGISTER_NUM],
    pub(crate) index_register: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    pub(crate) keys: [bool; NUM_KEYS],
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
}

// The contents of rom are copied to address 512 in ram
const START_ADDRESS: u16 = 0x200;

impl Emulation {
    pub fn new() -> Self {
        let mut new_emulation = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            frame_buffer: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            registers: [0; REGISTER_NUM],
            index_register: 0,
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
        self.frame_buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.registers = [0; REGISTER_NUM];
        self.index_register = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..font::SET_SIZE].copy_from_slice(&font::SET);
    }

    // returns a pointer to the frame buffer
    pub fn get_display(&self) -> &[bool] {
        &self.frame_buffer
    }

    // sets keys in the key array
    pub fn key_press(&mut self, index: usize, pressed: bool) {
        self.keys[index] = pressed;
    }

    // load ROM data into RAM from start address
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    // pushes and popping values to and from the stack
    pub(crate) fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }
    pub(crate) fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    pub(crate) fn next_instruction(&mut self){
        self.program_counter += INSTRUCTION_SIZE;
    }

    // CPU instructions
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        cpu::execute(self, op);
    }

    // fetches the next cpu instruction for execution
    // all instructions are 2 bytes -> returns 16-bit opcode combined as Big Endian
    fn fetch(&mut self) -> u16 {
        let first_byte = self.ram[self.program_counter as usize] as u16;
        let second_byte = self.ram[(self.program_counter + 1) as usize] as u16;
        let op = (first_byte << 8) | second_byte;
        self.next_instruction();
        op
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