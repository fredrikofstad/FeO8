use feo_core;
use feo_core::Emulation;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;


const SCALE: u32 = 15;
const WIDTH: u32 = (feo_core::SCREEN_WIDTH as u32) * SCALE;
const HEIGHT: u32 = (feo_core::SCREEN_HEIGHT as u32) * SCALE;

pub fn init(mut emu: &mut Emulation) {
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
        emu.tick();
        draw_screen(emu, &mut canvas);
    }
}

fn draw_screen(emu: &Emulation, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let screen_buf = emu.get_display();
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % feo_core::SCREEN_WIDTH) as u32;
            let y = (i / feo_core::SCREEN_WIDTH) as u32;
    // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
