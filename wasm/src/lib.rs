use feo_core::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use js_sys::Uint8Array;


#[wasm_bindgen]
pub struct EmulatorWasm {
    emu: Emulation,
    ctx: CanvasRenderingContext2d,
}

// Bindings for JS
#[wasm_bindgen]
impl EmulatorWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EmulatorWasm, JsValue>  {
        let emu = Emulation::new();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let ctx = canvas.get_context("2d")?.unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        Ok(EmulatorWasm{emu, ctx})
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.emu.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.emu.tick_timers();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emu.reset();
    }

    #[wasm_bindgen]
    pub fn key_press(&mut self, event: KeyboardEvent, pressed: bool) {
        let key = event.key();
        if let Some(key_code) = key_map(&key) {
            self.emu.key_press(key_code, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load(&mut self, rom: Uint8Array) {
        self.emu.load(&rom.to_vec());
    }

    #[wasm_bindgen]
    pub fn render(&mut self, scale: usize) {
        let display = self.emu.get_display();
        for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
            if display[i] {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64
                );
            }
        }

    }

}

fn key_map(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
