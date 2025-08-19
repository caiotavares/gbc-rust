use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::*;

const CLOCK: f32 = 8.388608;

enum Register {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
struct Registers {
    af: u16, // Accumulator & Flags register
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
}

impl Registers {
    pub fn init() -> Registers {
        // Initial values for registers obtained from Pandocs
        Registers {
            af: 0x1180,
            bc: 0x0000,
            de: 0xFF56,
            hl: 0x000D,
            pc: 0x0100, // ROM data starts at 0x0100, ignoring the bootloader checks
            sp: 0xFFFE,
        }
    }

    pub fn set_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        let current_state = self.af.split().1;
        let new_state: u8 = 0x00;
    }

    pub fn from_enum(&mut self, reg: Register) -> &mut u16 {
        match reg {
            Register::AF => &mut self.af,
            Register::BC => &mut self.bc,
            Register::DE => &mut self.de,
            Register::HL => &mut self.hl,
            Register::SP => &mut self.sp,
            Register::PC => &mut self.pc,
        }
    }
}

struct Clock {
    cycles: u8,
    clock_speed: usize,
}

impl Clock {
    pub fn init() -> Clock {
        Clock {
            cycles: 0,
            clock_speed: CLOCK as usize,
        }
    }
}

struct ALU {}

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
        /// TODO: Should this be constrained according to CPU clock?
        loop {
            let data = CPU::fetch(&mut self.registers.pc, &self.memory);
            let ins = Instruction::decode(data);
            if ins == Instruction::STOP {
                break;
            } else {
                self.execute(ins);
            }
        }
    }

    fn add_r8_r8(&mut self, source: u8, dest: &mut u8) {}

    fn inc_r8(data: u8) -> u8 {
        let result: u8;
        if data == 0xFF {
            result = 0x00;
        } else {
            result = data + 1;
        }
        result
    }

    fn inc_r8_lsb(&mut self, reg: Register) {
        let register = self.registers.from_enum(reg);
        let lsb = register.split().0;
        let new = CPU::inc_r8(lsb);
        if new == 0x00 {
            self.registers
                .set_flags(Some(true), Some(false), Some(true), None);
        } else {
            self.registers.set_flags(None, Some(false), None, None);
        }
        *register = *register | (new as u16);
        self.clock.cycles += 1;
    }

    fn inc_r8_msb(&mut self, reg: Register) {
        let register = self.registers.from_enum(reg);
        if *register == 0xFF {
            *register = 0x00;
            self.registers
                .set_flags(Some(true), Some(false), Some(true), None);
        } else {
            *reg += 1;
            self.registers.set_flags(None, Some(false), None, None);
        }
        self.clock.cycles += 1;
    }

    fn load_r8_lsb(&mut self, reg: Register) {
        let data = CPU::fetch(&mut self.registers.pc, &self.memory);
        let register = self.registers.from_enum(reg);
        *register = *register | (data as u16);
        self.clock.cycles += 2;
    }

    fn load_r8_msb(&mut self, reg: Register) {
        let data = CPU::fetch(&mut self.registers.pc, &self.memory);
        let register = self.registers.from_enum(reg);
        *register = *register | ((data as u16) << 8);
        self.clock.cycles += 2;
    }

    fn load_r16(&mut self, reg: Register) {
        let lsb = CPU::fetch(&mut self.registers.pc, &self.memory);
        let msb = CPU::fetch(&mut self.registers.pc, &self.memory);
        let register = self.registers.from_enum(reg);
        *register = as_u16(lsb, msb);
        self.clock.cycles += 3;
    }

    fn fetch(pc: &mut u16, memory: &Memory) -> u8 {
        let data = memory.read(*pc);
        *pc += 1;
        data
    }

    fn execute(&mut self, ins: Instruction) {
        match ins {
            // Control
            Instruction::NOP => self.clock.cycles += 1,
            // Load 8 bits
            Instruction::LD_A_u8 => self.load_r8_lsb(Register::AF),
            Instruction::LD_B_u8 => self.load_r8_lsb(Register::BC),
            Instruction::LD_C_u8 => self.load_r8_msb(Register::BC),
            Instruction::LD_D_u8 => self.load_r8_lsb(Register::DE),
            Instruction::LD_E_u8 => self.load_r8_msb(Register::DE),
            Instruction::LD_H_u8 => self.load_r8_lsb(Register::HL),
            Instruction::LD_L_u8 => self.load_r8_msb(Register::HL),

            // Load 16 bits
            Instruction::LD_BC_u16 => self.load_r16(Register::BC),
            Instruction::LD_DE_u16 => self.load_r16(Register::DE),
            Instruction::LD_HL_u16 => self.load_r16(Register::HL),

            // INC
            Instruction::INC_E => self.inc_r8_msb(Register::DE),

            Instruction::LD_BC_A => {
                self.memory
                    .write(self.registers.bc, self.registers.af.split().0);
                self.clock.cycles += 2;
            }

            Instruction::LD_DE_A => {
                self.memory
                    .write(self.registers.de, self.registers.af.split().0);
                self.clock.cycles += 2;
            }

            Instruction::Invalid => {}

            _ => {}
        }
    }
}
