#![allow(non_camel_case_types)]

use crate::*;

#[derive(Debug)]
enum Instruction {
    // Load
    LD_B_d8,
    LD_C_d8,
    LD_BC_d16,
    LD_BC_A,
    LD_A_BC,
    LD_A_DE,
    LD_D_d8,
    LD_E_d8,
    LD_DE_d16,
    LD_DE_A,

    LD_a16_SP,

    // Increment
    INC_B,
    INC_C,
    INC_BC,
    INC_D,
    INC_E,
    INC_DE,

    // Decrement
    DEC_B,
    DEC_C,
    DEC_BC,
    DEC_D,
    DEC_E,
    DEC_DE,

    // Rotation & Shifts
    RLCA,
    RRCA,
    RLA,
    RRA,

    // Addition
    ADD_HL_BC,
    ADD_HL_DE,

    // Stop/Halt
    STOP,
    NOP,

    // Jumps
    JR_r8,
    JP_a16,

    // Unknown
    Invalid,
}

impl Instruction {
    fn decode(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,
            0x01 => Instruction::LD_BC_d16,
            0x02 => Instruction::LD_BC_A,
            0x03 => Instruction::INC_BC,
            0x04 => Instruction::INC_B,
            0x05 => Instruction::DEC_B,
            0x06 => Instruction::LD_B_d8,
            0x07 => Instruction::RLCA,
            0x08 => Instruction::LD_a16_SP,
            0x09 => Instruction::ADD_HL_BC,
            0x0A => Instruction::LD_A_BC,
            0x0B => Instruction::DEC_BC,
            0x0C => Instruction::INC_C,
            0x0D => Instruction::DEC_C,
            0x0E => Instruction::LD_C_d8,
            0x0F => Instruction::RRCA,

            0x10 => Instruction::STOP,
            0x11 => Instruction::LD_DE_d16,
            0x12 => Instruction::LD_DE_A,
            0x13 => Instruction::INC_DE,
            0x14 => Instruction::INC_D,
            0x15 => Instruction::DEC_D,
            0x16 => Instruction::LD_D_d8,
            0x17 => Instruction::RLA,
            0x18 => Instruction::JR_r8,
            0x19 => Instruction::ADD_HL_DE,
            0x1A => Instruction::LD_A_DE,
            0x1B => Instruction::DEC_DE,
            0x1C => Instruction::INC_E,
            0x1D => Instruction::DEC_E,
            0x1E => Instruction::LD_E_d8,
            0x1F => Instruction::RRA,

            _ => Instruction::Invalid
        }
    }
}

#[derive(Debug)]
struct Registers {
    a: u8,
    /// Flags register
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
}

impl Registers {
    pub fn init() -> Registers {
        // Initial values for registers obtained from Pandocs
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100, // ROM data starts at 0x0100, ignoring the bootloader checks
            sp: 0xFFFE,
        }
    }

    pub fn read_af(&self) -> u16 {
        unsigned_16(self.f, self.a)
    }

    pub fn read_bc(&self) -> u16 {
        unsigned_16(self.c, self.b)
    }

    pub fn read_de(&self) -> u16 {
        unsigned_16(self.e, self.d)
    }

    pub fn read_hl(&self) -> u16 {
        unsigned_16(self.l, self.h)
    }

    pub fn write_af(&mut self, data: u16) {
        let u8s = data.split();
        self.f = u8s.0;
        self.a = u8s.1;
    }

    pub fn write_bc(&mut self, data: u16) {
        let u8s = data.split();
        self.c = u8s.0;
        self.b = u8s.1;
    }

    pub fn write_de(&mut self, data: u16) {
        let u8s = data.split();
        self.e = u8s.0;
        self.d = u8s.1;
    }

    pub fn write_hl(&mut self, data: u16) {
        let u8s = data.split();
        self.l = u8s.0;
        self.h = u8s.1;
    }
}

struct Clock {
    cycles: u8,
}

impl Clock {
    pub fn init() -> Clock {
        Clock { cycles: 0 }
    }
}

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

pub struct CPU {
    memory: Memory,
    registers: Registers,
    clock: Clock,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            memory,
            registers: Registers::init(),
            clock: Clock::init(),
        }
    }

    pub fn init(&mut self) {
        loop {
            let data = self.fetch();
            let ins = Instruction::decode(data);
            self.execute(ins);
        }
    }

    fn fetch(&mut self) -> u8 {
        let data = self.memory.read(self.registers.pc);
        self.registers.pc += 1;
        data
    }

    fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::NOP => {
                self.clock.cycles += 1
            }
            Instruction::STOP => {}
            Instruction::LD_B_d8 => {
                let data = self.fetch();
                self.registers.b = data;
                self.clock.cycles += 2;
            }

            Instruction::LD_BC_d16 => {
                let data = unsigned_16(self.fetch(), self.fetch());
                self.registers.write_bc(data);
                self.clock.cycles += 3;
            }

            Instruction::LD_BC_A => {
                self.memory.write(self.registers.read_bc(), self.registers.a);
                self.clock.cycles += 2;
            }

            Instruction::Invalid => {}
            _ => {}
        }
    }
}
