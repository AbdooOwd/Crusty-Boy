use crate::{gpu::{self, TilePixelValue}, utils::log};

pub const NIN: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
    // half-split
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

const b_tile: [u8; 16] = [
    0x3C, 0x7E, 
    0x42, 0x42, 
    0x42, 0x42, 
    0x42, 0x42, 
    0x7E, 0x5E, 
    0x7E, 0x0A, 
    0x7C, 0x56, 
    0x38, 0x7C
];

const a_tile: [u8; 16] = [
    0x7C, 0x7C,
    0x00, 0xC6,
    0xC6, 0x00,
    0x00, 0xFE,
    0xC6, 0xC6,
    0x00, 0xC6,
    0xC6, 0x00,
    0x00, 0x00
];

pub fn play() {
    // 0-23

    for (i, byte) in NIN.iter().enumerate() {
        let hnibble = (byte & 0xF0) >> 4;
        let lnibble = byte & 0x0F;

        for bit_i in 0..4 {
            let mask = 1 << (3 - bit_i);

            let msb = hnibble & mask;
            let lsb = lnibble & mask;

            match (msb != 0, lsb != 0) {
                (true, true) => print!("█"),
                (true, false) => print!("▒"),
                (false, true) => print!("░"),
                (false, false) => print!(" "),
            }
        }
    }
}