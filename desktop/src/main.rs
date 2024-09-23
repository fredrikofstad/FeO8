use feo_core::Emulation;

mod display;

fn main() {
    let mut emu = Emulation::new();
    display::init();
    /*
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: path to game rom unspecified");
        return;
    } */
}