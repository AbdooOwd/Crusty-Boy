pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x97FF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

pub const TILE_COUNT: usize = VRAM_SIZE / 0x10;


#[derive(Copy, Clone, Debug)]
pub enum TilePixelValue {
    White,
    Black,
    LightGray,
    DarkGray
}


pub type Tile = [[TilePixelValue; 8]; 8];


pub struct GPU {
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

    /*
    CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
    00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
    BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
    */

    pub fn get_nintendo_logo_data(data: &Vec<u8>) -> Vec<u8> {
        let mut logo: Vec<u8> = vec![0; 48];

        for address in 0x104..=0x133 {
            logo[address - 0x104] = data[address];
        }

        logo
    }
}