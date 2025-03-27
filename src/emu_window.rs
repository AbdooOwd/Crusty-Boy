use minifb::{Key, Window, WindowOptions};
use crate::{cpu::CPU, gpu::{self, TilePixelValue}};

const SCREEN_WIDTH: usize = 160 * SCREEN_MAGNIFIER;
const SCREEN_HEIGHT: usize = 144 * SCREEN_MAGNIFIER;
const SCREEN_MAGNIFIER: usize = 3;

pub fn window_life(mut cpu: CPU) {
    let mut window = Window::new(
        "Crusty-Boy",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .expect("Could not create window");
    let mut framebuffer = vec![0u32; SCREEN_WIDTH * SCREEN_HEIGHT]; // 1 pixel = 4 bytes (RGBA)

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_vram_to_framebuffer(&cpu.mem_bus.gpu, &mut framebuffer);
        // cpu.step();

        // Update the window with the pixel buffer
        window.update_with_buffer(&framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}

fn draw_vram_to_framebuffer(gpu: &gpu::GPU, framebuffer: &mut [u32]) {
    for tile_y in 0..18 { // 18 tiles fit vertically in 144px
        for tile_x in 0..20 { // 20 tiles fit horizontally in 160px
            let tile = gpu.tileset[(tile_y * 20 + tile_x) % 384]; // Get a tile

            for row in 0..8 {
                for col in 0..8 {
                    let pixel = tile[row][col]; // Get pixel value
                    let color = match pixel {
                        TilePixelValue::Black => 0x000000,  // Black
                        TilePixelValue::DarkGray => 0x555555,  // Dark Gray
                        TilePixelValue::LightGray => 0xAAAAAA,  // Light Gray
                        TilePixelValue::White => 0xFFFFFF, // White
                    };

                    let x = tile_x * 8 + col;
                    let y = tile_y * 8 + row;
                    let index = y * SCREEN_WIDTH + x;
                    framebuffer[index] = color;
                }
            }
        }
    }
}