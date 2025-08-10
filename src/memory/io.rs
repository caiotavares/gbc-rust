#[derive(Debug)]
pub struct IO {
    // 0xFF00
    joypad: [u8; 1],
    // 0xFF01 ~ 0xFF02
    serial: [u8; 2],
    // 0xFF04 ~ 0xFF07
    timer: [u8; 4],
    // 0xFF0F
    interrupts: [u8; 1],
    // 0xFF10 ~ 0xFF26
    audio: [u8; 22],
    // 0xFF30 ~ 0xFF3F
    wave: [u8; 16],
    // 0xFF40 ~ 0xFF4B
    lcd: [u8; 11],
    // 0xFF4F
    vram_bank_select: [u8; 1],
    // 0xFF50
    boot_rom: [u8; 1],
    // 0xFF51 ~ 0xFF55
    vram_dma: [u8; 5],
    // 0xFF68 ~ 0xFF6B
    obj_palettes: [u8; 3],
    // 0xFF70
    wram_bank_select: [u8; 1],
}

impl IO {
    pub fn init() -> IO {
        IO {
            joypad: [0; 1],
            serial: [0; 2],
            timer: [0; 4],
            interrupts: [0; 1],
            audio: [0; 22],
            wave: [0; 16],
            lcd: [0; 11],
            vram_bank_select: [0; 1],
            boot_rom: [0; 1],
            vram_dma: [0; 5],
            obj_palettes: [0; 3],
            wram_bank_select: [0; 1],
        }
    }

    
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0xFF00 => self.joypad[0],
            0xFF01..=0xFF02 => self.serial[address - 0xFF01],
            0xFF04..=0xFF07 => self.timer[address - 0xFF04],
            0xFF0F => self.interrupts[0],
            0xFF10..=0xFF26 => self.audio[address - 0xFF10],
            0xFF30..=0xFF3F => self.wave[address - 0xFF30],
            0xFF40..=0xFF4B => self.lcd[address - 0xFF40],
            0xFF4F => self.vram_bank_select[0],
            0xFF50 => self.boot_rom[0],
            0xFF51..=0xFF55 => self.vram_dma[address - 0xFF51],
            0xFF68..=0xFF6B => self.obj_palettes[address - 0xFF68],
            0xFF70 => self.wram_bank_select[0],
            _ => 0,
        }
    }

    pub fn write(&self, address: usize, data: u8) {

    }
}
