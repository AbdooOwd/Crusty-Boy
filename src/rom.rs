use std::{fs::File, io::Read};

use crate::utils::log;

const ROM_TYPE_BYTE_POS: usize = 0x0147;
const ROM_SIZE_BYTE_POS: usize = 0x0147;
const ROM_REGION_BYTE_POS: usize = 0x014A;
const ROM_VERSION_BYTE_POS: usize = 0x014C;
const ROM_HEADER_CHECKSUM_RANGE: std::ops::RangeInclusive<usize> = 0x134..=0x14C;

#[derive(Clone)]
pub struct ROM {
    pub name: String,
    pub size: usize,
    // TODO: RAM Size
    pub data: Vec<u8>,
    pub cartridge_type: u8,
    pub region: &'static str,
    pub version: u8,
    pub header_checksum: u8,
    // TODO: Manufacturer Code
    // TODO: CGB Flag
    // TODO: New + Old Licensee Code
    // TODO SGB Flag
    // TODO: 
}


impl ROM {

    #[allow(unused)]
    /**
        This is a dangerous function! If you don't make sure to initialize the values, 
        you'll be running the rest of the program with a fake rom!
     */
    pub fn new() -> Self {
        ROM {
            name: String::from("UNKNOWN"),
            size: 0,
            data: vec![0; 0xFFFF],
            cartridge_type: 0,
            region: "NOWHERE",
            version: 0,
            header_checksum: 0
        }
    }

    pub fn read_rom(path: &str) -> Self {
        let mut rom_file: File = File::open(path).expect("Couldn't open ROM");
        let mut buffer = vec![0; 0xFFFF];
        let buffer_len = buffer.len();

        if buffer_len <= 0 {
            panic!("Buffer length is zero!");
        }
    
        let _bytes_count: usize = rom_file.read(&mut buffer).expect("Couldn't read ROM");
    
        ROM {
            name: ROM::get_rom_name(&buffer),
            size: ROM::get_rom_size(buffer[ROM_SIZE_BYTE_POS]),
            data: buffer.clone(),
            cartridge_type: buffer[ROM_TYPE_BYTE_POS],
            region: ROM::get_region(buffer[ROM_REGION_BYTE_POS]),
            version: buffer[ROM_VERSION_BYTE_POS],
            header_checksum: ROM::get_header_checksum(&buffer),
        }
    }

    pub fn get_header_checksum(data: &Vec<u8>) -> u8 {
        let mut checksum: u8 = 0;

        for address in ROM_HEADER_CHECKSUM_RANGE {
            checksum = checksum.wrapping_sub(data[address]).wrapping_sub(1);
        }

        checksum
    }

    pub fn get_region(region_byte: u8) -> &'static str {
        match region_byte {
            0x00 => "JAPAN",
            0x01 => "WORLD",
            _ => "UNKNOWN",
        }
    }

    pub fn get_rom_size(size_byte: u8) -> usize {
        32 * (1 << size_byte)
    }

    pub fn get_cartridge_type_name(type_byte: u8) -> &'static str {
        match type_byte {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0B => "MMM01",
            0x0C => "MMM01+RAM",
            0x0D => "MMM01+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY 10",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            0x1C => "MBC5+RUMBLE",
            0x1D => "MBC5+RUMBLE+RAM",
            0x1E => "MBC5+RUMBLE+RAM+BATTERY",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "BANDAI TAMA5",
            0xFE => "HuC3",
            0xFF => "HuC1+RAM+BATTERY",

            _ => {
                log(&format!("Unrecognized Cartridge Type: {type_byte}"));
                "UnkownType"
            }
        }
    }
    
    pub fn get_rom_name(data: &Vec<u8>) -> String {
        let mut name_buffer: String = String::new();
        // TODO: support CGB flag

        for i in 0..16 {
            let char_byte = data[i + 0x0134];

            if char_byte == 0 {
                break;
            }

            name_buffer.push(char_byte as char);
        }

        name_buffer
    }
}