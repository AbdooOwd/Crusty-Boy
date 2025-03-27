use crate::{registers::LCDControl, utils::panic_log};

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x97FF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

pub const TILE_MAP_START: usize = 0x9800;
pub const TILE_MAP_END: usize = 0x9BFF;
pub const TILE_MAP_SIZE: usize = TILE_MAP_END - TILE_MAP_START + 1;

pub const TILE_COUNT: usize = VRAM_SIZE / 0x10;

/* NINTENDO LOGO
CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
*/

#[derive(Copy, Clone, Debug)]
pub enum TilePixelValue {
    White,
    Black,
    LightGray,
    DarkGray
}


pub type Tile = [[TilePixelValue; 8]; 8];


pub struct GPU {
    // should start at 0x8000 in gameboy memory
    pub vram: [u8; VRAM_SIZE],

    // 0x1800 (vram available for tiles) / 0x10 (size of one tiles) = 0x180 = 384 (total available tile spots)
    pub tileset: [Tile; TILE_COUNT],
}


impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            tileset: [[[TilePixelValue::White; 8]; 8]; TILE_COUNT],
        }
    }

    pub fn initialize_graphics(&self, lcdc: &mut LCDControl) -> () {
        *lcdc = LCDControl::new();  // default LCDC
    }

    pub fn get_tile(&self, tile_index: usize, area_method: bool) -> Tile {
        if tile_index > 255 {
            panic_log(&format!("[GPU] Incorrect tile index {tile_index} (must be 0-255)"));
        }

        let vram_index: usize;

        if area_method {    
            // 0x8000 method
            // thus, can only get tiles from block 0 & 1

            if (0..=127).contains(&tile_index) {
                // block 0
                vram_index = tile_index * 16;
            } else {
                // block 1
                vram_index = 0x0800 + tile_index * 16;
            }
        } else {
            // 0x8800 method
            // thus, can only get tiles from block 1 & 2

            if (0..=127).contains(&tile_index) {
                // block 2
                vram_index = 0x1000 + tile_index * 16;
            } else {
                // block 1
                vram_index = 0x0800 + (tile_index - 128) * 16;
            }
        }

        let tile_bytes: &[u8] = &self.vram[vram_index..vram_index + 16*16];
        let tile = TileManipulation::tile_from_bytes(tile_bytes.to_vec());

        tile
    }

    pub fn vram_read_byte(&self, addr: usize) -> u8 {
        self.vram[addr]
    }

    pub fn vram_write_byte(&mut self, index: usize, value: u8) -> () {
        self.vram[index] = value;

        if index >= 0x1800 { return }

        let normalized_index = index & 0xFFFE;  // takes the even index (% 2 = 0)

        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        let tile_index = index / 16;
        let row_index = (index % 16) / 2;   // how many 16s there are? & each row has 2 bytes? (we only need one byte!!!)

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);  // get the backward (from the left) bit index

            // TODO: HOW THE HECK DO I KNOW WHO'S THE "LSB" AND WHO's THE "MSB"!?

            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let pixel_value = match (msb != 0, lsb != 0) {
                (true, true) => TilePixelValue::White,
                (true, false) => TilePixelValue::DarkGray,
                (false, true) => TilePixelValue::LightGray,
                (false, false) => TilePixelValue::Black
            };

            self.tileset[tile_index][row_index][pixel_index] = pixel_value;
        }
    }
}


#[allow(non_snake_case)]
pub mod TileManipulation {
    use super::{Tile, TilePixelValue};

    pub fn tile_from_bytes(tile_bytes: Vec<u8>) -> Tile {
        let mut tile: Tile = [[TilePixelValue::White; 8]; 8];

        for mut indexer in 0..16 {
            indexer = indexer & 0xFFFE;

            let byte1 = tile_bytes[indexer];
            let byte2 = tile_bytes[indexer + 1];

            for bit_i in 0..8 {
                let mask = 1 << (7 - bit_i);

                let lsb = byte1 & mask;
                let msb = byte2 & mask;

                let pixel_value = match (msb != 0, lsb != 0) {
                    (true, true) => TilePixelValue::Black,
                    (true, false) => TilePixelValue::DarkGray,
                    (false, true) => TilePixelValue::LightGray,
                    (false, false) => TilePixelValue::White,
                };

                tile[indexer / 2][bit_i] = pixel_value;
            }
        }

        tile
    }

    pub fn tile_to_string(tile_data: &Tile) -> String {
        let mut s: String = String::new();

        for ty in 0..8 {
            for tx in 0..8 {
                let c = match tile_data[ty][tx] {
                    TilePixelValue::Black => '█',
                    TilePixelValue::DarkGray => '▒',
                    TilePixelValue::LightGray => '░',
                    TilePixelValue::White => ' ',
                };
    
                s.push(c);
            }
            s.push('\n');
        }

        s
    }
}