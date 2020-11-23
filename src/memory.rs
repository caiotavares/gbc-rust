use crate::_32KB;

#[derive(Debug)]
pub struct Memory {
    // 0x0000 ~ 0x7FFF
    rom: [u8; _32KB],
    // 0x8000 ~ 0xFFFF
    ram: [u8; _32KB],
    // TODO: Implement all memory regions
}

impl Memory {
    pub fn new(program: [u8; _32KB]) -> Memory {
        Memory {
            // Over-simplified to enable us to move on
            rom: program,
            ram: [0; _32KB], // TODO: Should we use Vec<u8> instead of [u8; _32KB]?
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        // TODO: Access to some regions should be protected
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            _ => self.ram[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // TODO: Access to some regions should be protected
        match address {
            // ROM data
            0x0000..=0x7FFF => panic!("Forbidden write into ROM memory region!"),
            _ => self.ram[address as usize] = data
        }
    }
}
