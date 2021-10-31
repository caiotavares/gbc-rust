use crate::{_32KB, _8KB, _4KB};

pub struct IO {
    memory_range: [u8; 127],
    // TODO: Set each peripheral its own memory address
}

#[derive(Debug)]
pub struct Memory {
    /// 0x0000 ~ 0x7FFF Bank 0~NN
    rom: [u8; _32KB],
    /// 0x8000 ~ 0x9FFF
    vram: [u8; _8KB],
    /// 0xA000 ~ 0xBFFF External RAM
    ram: [u8; _8KB],
    /// 0xC000 ~ 0xCFFF WRAM
    wram1: [u8; _4KB],
    /// 0xD000 ~ 0xDFFF WRAM
    wram2: [u8; _4KB],
    /// 0xFF00 ~ 0xFF7F I/O Registers
    // io: [u8; 127],

}

impl Memory {
    pub fn new(program: [u8; _32KB]) -> Memory {
        Memory {
            // TODO: Should we use Vec<u8> instead of [u8; _32KB]?
            rom: program,
            vram: [0; _8KB],
            ram: [0; _32KB],
            wram1: [0; _4KB],
            wram2: [0; _4KB],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        // TODO: Read access to some regions should be protected
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            _ => self.ram[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // TODO: Write access to some regions should be protected
        match address {
            // ROM data
            0x0000..=0x7FFF => panic!("Forbidden write into ROM memory region!"),
            _ => self.ram[address as usize] = data
        }
    }
}
