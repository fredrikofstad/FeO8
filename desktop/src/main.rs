use std::env;
use feo_core::Emulation;
use std::fs::File;
use std::io::Read;


mod gui;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Path to game rom unspecified");
        return;
    }
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    let mut emu = Emulation::new();
    emu.load(&buffer);
    gui::init(&mut emu);
}