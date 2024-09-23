use feo_core;
use std::env;
use sdl2::event::Event;

const SCALE: u32 = 15;
const WIDTH: u32 = (feo_core::SCREEN_WIDTH as u32) * SCALE;
const HEIGHT: u32 = (feo_core::SCREEN_HEIGHT as u32) * SCALE;

pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("FeO8 - Chip-8 Emulator", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'game_loop;
                },
                _ => ()
            }
        }
    }

}