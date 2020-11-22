use gbc::_32KB;
use gbc::cpu::{CPU, Memory};

fn main() {
    let mut program: [u8; _32KB] = [0; _32KB]; // TODO: Read from file
    program[0x0100] = 0x06;
    program[0x0101] = 0xdd;
    let memory = Memory::new(program);
    let mut cpu = CPU::new(memory);
    cpu.init();
}
