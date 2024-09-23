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
        Self {
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
        }
    }
}


