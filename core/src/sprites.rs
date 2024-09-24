use crate::emulation;

/*
Sprites are always 8 pixels wide but can be a variable amount of pixels from 1 to 16 pixels tall
Height is specified in the fourth nibble (N) of the DXYN opcode
The sprites are stored row by row beginning with the address stored in register I.
Register F is set when bits are flipped (from black pixel to white or vice versa)
*/

pub fn draw_sprite(emu: &mut emulation::Emulation, nibble2:u16, nibble3:u16, nibble4:u16){
    // nibble 2 and 3 specify the x and y coordinates to draw to
    let screen_x = emu.registers[nibble2 as usize] as u16;
    let screen_y = emu.registers[nibble3 as usize] as u16;
    // nibble 4 specifies the height of the sprite
    let sprite_height = nibble4;

    let mut flipped_pixels = false;
    // draw pixels line by line
    for line in 0..sprite_height {
        // get the address where the sprite line is stored
        let address = emu.index_register + line;
        let pixels = emu.ram[address as usize];
        // Iterate over each pixel in the line
        for pixel in 0..8 {
            // Use a mask to fetch current pixel's bit. Only flip if a 1
            if (pixels & (0b1000_0000 >> pixel)) != 0 {
                // apply modulo to allow sprites to wrap around screen
                let x = (screen_x + pixel) as usize % emulation::SCREEN_WIDTH;
                let y = (screen_y + line) as usize % emulation::SCREEN_HEIGHT;
                // Get our pixel's index for our 1D screen array
                let index = x + emulation::SCREEN_WIDTH * y;
                // Check if we're about to flip the pixel and set
                flipped_pixels |= emu.frame_buffer[index];
                emu.frame_buffer[index] ^= true;
            }
        }
    }
    emu.registers[0xF] = if flipped_pixels {1} else {0};

}