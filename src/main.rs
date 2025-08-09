use gbc::cpu::CPU;
use gbc::memory::Memory;
use gbc::_32KB;
use std::fs;

fn read_program() -> [u8; _32KB] {
    let program = fs::read("./tests/fixtures/program.bin").expect("Program is unreadable");
    let size: usize = program.len();
    if size > _32KB {
        panic!("Program is too large, maximum size is {} bytes", _32KB);
    }
    let mut buffer: [u8; _32KB] = [0; _32KB];
    buffer[..size].copy_from_slice(&program);
    buffer
}

fn main() {
    let program: [u8; _32KB] = read_program();
    let memory = Memory::new(program);
    let mut cpu = CPU::new(memory);
    cpu.init();
}
