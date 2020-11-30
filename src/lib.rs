pub mod cpu;
pub mod memory;
pub mod instructions;

pub const _128KB: usize = 128 * _1KB;
pub const _64KB: usize = 64 * _1KB;
pub const _32KB: usize = 32 * _1KB;
pub const _16KB: usize = 16 * _1KB;
pub const _8KB: usize = 8 * _1KB;
pub const _4KB: usize = 4 * _1KB;
pub const _2KB: usize = 2 * _1KB;
pub const _1KB: usize = 1024;

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
