pub mod cpu;
pub mod memory;
pub mod instructions;

pub const _64KB: usize = 65536;
pub const _32KB: usize = 32767;
pub const _16KB: usize = 16384;
pub const _8KB: usize = 8192;
pub const _4KB: usize = 4096;

pub fn unsigned_16(lsb: u8, msb: u8) -> u16 {
    let value: u16 = 0x0000;
    (value | lsb) | (msb << 8)
}

pub trait Splitable {
    fn split(&self);
}

impl Splitable for u16 {
    fn split(&self) -> (u8, u8) {
        let msb: u8 = (self >> 8) as u8;
        let lsb: u8 = self as u8;
        (lsb, msb)
    }
}
