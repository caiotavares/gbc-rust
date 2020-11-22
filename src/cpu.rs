#![allow(non_camel_case_types)]

enum Instruction {
    // Load
    LD_B_d8,
    LD_C_d8,
    LD_BC_nn,
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

struct Registers {
    a: u8,
    flags: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

struct CPU {
    registers: Registers
}

impl CPU {
    pub fn init() {

    }

    fn fetch() -> Instruction {

    }

    fn execute(ins: Instruction) {
        match ins {
            Instruction::NOP => {}
            Instruction::LD_BC_A => {}
            Instruction::LD_BC_nn => {}
            Instruction::STOP => {}
            Instruction::Invalid => {}
            _ => {}
        }
    }
}
