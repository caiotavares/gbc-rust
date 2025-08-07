use crate::*;
use crate::memory::Memory;
use crate::instructions::Instruction;

const CLOCK: f32 = 8.388608;

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
        /// Initial values for registers obtained from Pandocs
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

    pub fn set_flags(&self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        let current_state = self.f;
        let new_state: u8 = 0x00;
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
    clock_speed: usize
}

impl Clock {
    pub fn init() -> Clock {
        Clock { cycles: 0, clock_speed: CLOCK as usize }
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
        /// TODO: Should this be constrained according to CPU clock?
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

            Instruction::LD_B_u8 => {
                self.registers.b = self.fetch();
                self.clock.cycles += 2;
            }

            Instruction::LD_C_u8 => {
                self.registers.c = self.fetch();
                self.clock.cycles += 2;
            }

            Instruction::LD_BC_u16 => {
                let data = unsigned_16(self.fetch(), self.fetch());
                self.registers.write_bc(data);
                self.clock.cycles += 3;
            }

            Instruction::LD_BC_A => {
                self.memory.write(self.registers.read_bc(), self.registers.a);
                self.clock.cycles += 2;
            }

            Instruction::LD_D_u8 => {
                self.registers.d = self.fetch();
                self.clock.cycles += 2;
            }

            Instruction::LD_E_u8 => {
                self.registers.e = self.fetch();
                self.clock.cycles += 2;
            }

            Instruction::LD_DE_u16 => {
                let data = unsigned_16(self.fetch(), self.fetch());
                self.registers.write_de(data);
                self.clock.cycles += 3;
            }

            Instruction::LD_DE_A => {
                self.memory.write(self.registers.read_de(), self.registers.a);
                self.clock.cycles += 2;
            }

            Instruction::INC_E => {
                if self.registers.e == 0xFF {
                    self.registers.e = 0x00;
                    self.registers.set_flags(Some(true), Some(false), Some(true), None);
                } else {
                    self.registers.e += 1;
                    self.registers.set_flags(None, Some(false), None, None);
                }
                self.clock.cycles += 1;
            }

            Instruction::Invalid => {}

            _ => {}
        }
    }
}
