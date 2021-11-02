use gbc::_32KB;
use gbc::cpu::CPU;
use gbc::memory::Memory;
use gbc::display::winit;
use std::fs;

fn read_program() -> Vec<u8> {
    fs::read("./tests/fixtures/program.bin").expect("Could not read program data")
}

fn main() {
    winit::init();
    let mut program: [u8; _32KB] = [0; _32KB]; // TODO: Read from file
    let memory = Memory::new(program);
    let mut cpu = CPU::new(memory);
    // cpu.init();
}
