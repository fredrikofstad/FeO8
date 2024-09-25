import init, * as wasm from "wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
const TICKS_PER_FRAME = 10

let frame = 0

const canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE
const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)
const input = document.getElementById("fileinput")

async function run() {
    await init()
    let emu = new wasm.EmulatorWasm()
    document.addEventListener("keydown", function(evt) {
        emu.key_press(evt, true)
    })
    document.addEventListener("keyup", function(evt) {
        emu.key_press(evt, false)
    })
    input.addEventListener("change", function(evt) {
        // Handle file loading
    }, false)

    input.addEventListener("change", function(evt) {
        // Stop previous game from rendering, if one exists
        if (frame !== 0) {
            window.cancelAnimationFrame(frame)
        }
        let file = evt.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        // Load in game as Uint8Array, send to .wasm, start main loop
        let fr = new FileReader()
        fr.onload = function(e) {
            let buffer = fr.result
            const rom = new Uint8Array(buffer)
            emu.reset()
            emu.load(rom)
            mainloop(emu)
        }
        fr.readAsArrayBuffer(file)
    }, false)
}

function mainloop(emu) {
    // Only draw every few ticks
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
        emu.tick()
    }
    emu.tick_timers()
    // Clear the canvas before drawing
    ctx.fillStyle = "black"
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)
    // Set the draw color back to white before we render our frame
    ctx.fillStyle = "white"
    emu.render(SCALE)
    frame = window.requestAnimationFrame(() => {
        mainloop(emu)
    })
}


run().catch(console.error)
