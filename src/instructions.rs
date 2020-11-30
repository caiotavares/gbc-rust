#![allow(non_camel_case_types)]

#[derive(Debug)]
pub enum Instruction {
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

    LD_H_d8,
    LD_L_d8,
    LD_HL_d16,
    LD_HL_A_Plus,
    LD_HL_A_Minus,
    LD_A_HL_Plus,
    LD_A_HL_Minus,

    LD_a16_SP,

    // Increment
    INC_B,
    INC_C,
    INC_BC,

    INC_D,
    INC_E,
    INC_DE,

    INC_H,
    INC_L,
    INC_HL,

    // Decrement
    DEC_B,
    DEC_C,
    DEC_BC,

    DEC_D,
    DEC_E,
    DEC_DE,

    DEC_H,
    DEC_L,
    DEC_HL,

    // Something
    DAA,
    CPL,

    // Rotation & Shifts
    RLCA,
    RRCA,
    RLA,
    RRA,

    // Addition
    ADD_HL_BC,
    ADD_HL_DE,
    ADD_HL_HL,

    // Stop/Halt
    STOP,
    NOP,

    // Jumps
    JR_r8,
    JR_NZ_r8,
    JR_Z_r8,
    JP_a16,

    // Unknown
    Invalid,
}

impl Instruction {
    pub fn decode(byte: u8) -> Instruction {
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

            0x20 => Instruction::JR_NZ_r8,
            0x21 => Instruction::LD_HL_d16,
            0x22 => Instruction::LD_HL_A_Plus,
            0x23 => Instruction::INC_HL,
            0x24 => Instruction::INC_H,
            0x25 => Instruction::DEC_H,
            0x26 => Instruction::LD_H_d8,
            0x27 => Instruction::DAA,
            0x28 => Instruction::JR_Z_r8,
            0x29 => Instruction::ADD_HL_HL,
            0x2A => Instruction::LD_A_HL_Minus,
            0x2B => Instruction::DEC_HL,
            0x2C => Instruction::INC_L,
            0x2D => Instruction::DEC_L,
            0x2E => Instruction::LD_L_d8,
            0x2F => Instruction::CPL,

            _ => Instruction::Invalid
        }
    }
}
