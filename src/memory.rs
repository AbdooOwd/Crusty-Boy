use crate::gpu::{GPU, VRAM_START, VRAM_END};

pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
    pub gpu: GPU,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
            gpu: GPU::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        match addr {
            VRAM_START..VRAM_END => {
                return self.gpu.vram_read_byte(addr - VRAM_START);
            },

            _ => {
                return self.memory[addr as usize];
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;

        match addr {
            VRAM_START..VRAM_END => {
                return self.gpu.vram_write_byte(addr - VRAM_START, byte);
            },

            _ => {
                self.memory[addr as usize] = byte;
            }
        }
    }
}