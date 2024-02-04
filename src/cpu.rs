mod decode;
mod decode_prefix_cb;
mod execute;
mod inst;

use crate::memory::{MemoryIF, IE, IF};
use inst::{FlagReg, Inst, Reg16, Reg8};
use std::fmt;

type M = usize;

pub struct Cpu {
    clock_m: M,
    // t = 4m
    reg: Registers,
    // Clock for last instr
    m: M,
    // t = 4m
    ime: bool,
    flags: Flags,
    //halt: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            clock_m: 0,
            reg: Registers::new(),
            m: 0,
            ime: false,
            //halt: false,
            flags: Flags {
                halt: false,
                stop: false,
            },
        }
    }
    pub fn run(&mut self, memory: &mut impl MemoryIF) -> Result<u16, String> {
        self.clock_m += 1;

        if (self.clock_m >= self.m) || self.flags.halt || self.flags.stop {
            self.clock_m = 0;
            if !self.flags.halt && !self.flags.stop {
                let (inst, addvance) = decode::decode(self.reg.pc, memory)?;
                self.reg.pc += addvance;
                (self.m, self.flags) = self.reg.execute(inst, memory, &mut self.ime)?;
            }

            // Interrupts
            let i_flag = memory.read_byte(IF);
            let i_enable = memory.read_byte(IE);
            let masked = i_flag & i_enable;
            if masked != 0 {
                self.flags.halt = false;
                if self.ime {
                    self.ime = false;
                    self.m += 5;
                    _ = self
                        .reg
                        .execute(Inst::Push16(Reg16::PC), memory, &mut self.ime)?;
                    if masked & 0x01 != 0 {
                        // VBlank
                        memory.write_byte(IF, i_flag & !0x01);
                        self.reg.write_reg16(&Reg16::PC, 0x40);
                    } else if masked & 0x02 != 0 {
                        // LCD
                        memory.write_byte(IF, i_flag & !0x02);
                        self.reg.write_reg16(&Reg16::PC, 0x48);
                    } else if masked & 0x04 != 0 {
                        // Timer
                        memory.write_byte(IF, i_flag & !0x04);
                        self.reg.write_reg16(&Reg16::PC, 0x50);
                    } else if masked & 0x08 != 0 {
                        // Serial
                        memory.write_byte(IF, i_flag & !0x08);
                        self.reg.write_reg16(&Reg16::PC, 0x58);
                    } else if masked & 0x10 != 0 {
                        // Joypad
                        memory.write_byte(IF, i_flag & !0x10);
                        self.reg.write_reg16(&Reg16::PC, 0x60);
                    } else {
                        return Err(format!("Cpu::run: invalid if or ie"));
                    }
                }
            }
        }
        Ok(self.reg.pc)
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "clock_m: {}, m: {}, ime: {}, halt: {}, stop: {}\nreg:\n{}",
            self.clock_m, self.m, self.ime, self.flags.halt, self.flags.stop, self.reg
        )
    }
}

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0x100,
            sp: 0xfffe,
        }
    }
    fn read_reg8(&self, r: &Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }
    fn write_reg8(&mut self, r: &Reg8, v: u8) {
        match r {
            Reg8::A => self.a = v,
            Reg8::B => self.b = v,
            Reg8::C => self.c = v,
            Reg8::D => self.d = v,
            Reg8::E => self.e = v,
            Reg8::H => self.h = v,
            Reg8::L => self.l = v,
        }
    }
    fn read_reg16(&self, r: &Reg16) -> u16 {
        match r {
            Reg16::AF => {
                let a = self.a as u16;
                let f = self.f as u16;
                (a << 8) | f
            }
            Reg16::BC => {
                let b = self.b as u16;
                let c = self.c as u16;
                (b << 8) | c
            }
            Reg16::DE => {
                let d = self.d as u16;
                let e = self.e as u16;
                (d << 8) | e
            }
            Reg16::HL => {
                let h = self.h as u16;
                let l = self.l as u16;
                (h << 8) | l
            }
            Reg16::PC => self.pc,
            Reg16::SP => self.sp,
        }
    }
    fn write_reg16(&mut self, r: &Reg16, v: u16) {
        let v0 = (v >> 8) as u8;
        let v1 = (v & 0x00ff) as u8;
        match r {
            Reg16::AF => {
                self.a = v0;
                self.f = v1 & 0xf0; // bits 0 - 3 are always 0.
            }
            Reg16::BC => {
                self.b = v0;
                self.c = v1;
            }
            Reg16::DE => {
                self.d = v0;
                self.e = v1;
            }
            Reg16::HL => {
                self.h = v0;
                self.l = v1;
            }
            Reg16::PC => self.pc = v,
            Reg16::SP => self.sp = v,
        }
    }
    fn test_f(&self, f: FlagReg) -> bool {
        let mask = flag_mask(f);
        (self.f & mask) == mask
    }
    fn clear_f(&mut self, f: FlagReg) {
        let mask = flag_mask(f);
        self.f &= !mask;
    }
    fn set_f(&mut self, f: FlagReg) {
        let mask = flag_mask(f);
        self.f |= mask;
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AF: {:#04x} {:02x} ({:#010b} {:08b})\n",
            self.a, self.f, self.a, self.f
        )?;
        write!(f, " (")?;
        write!(f, "Z: {}, ", b2n(self.test_f(FlagReg::Z)))?;
        write!(f, "N: {}, ", b2n(self.test_f(FlagReg::N)))?;
        write!(f, "H: {}, ", b2n(self.test_f(FlagReg::H)))?;
        write!(f, "C: {}", b2n(self.test_f(FlagReg::C)))?;
        write!(f, ")\n")?;
        write!(
            f,
            "BC: {:#04x} {:02x} ({:#010b} {:08b})\n",
            self.b, self.c, self.b, self.c
        )?;
        write!(
            f,
            "DE: {:#04x} {:02x} ({:#010b} {:08b})\n",
            self.d, self.e, self.d, self.e
        )?;
        write!(
            f,
            "HL: {:#04x} {:02x} ({:#010b} {:08b})\n",
            self.h, self.l, self.h, self.l
        )?;
        write!(f, "PC: {:#04x}, ", self.pc)?;
        write!(f, "SP: {:#04x}\n", self.sp)
    }
}

fn flag_mask(f: FlagReg) -> u8 {
    match f {
        FlagReg::Z => 0x80,
        FlagReg::N => 0x40,
        FlagReg::H => 0x20,
        FlagReg::C => 0x10,
    }
}

fn b2n(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

#[derive(PartialEq, Debug)]
pub struct Flags {
    halt: bool,
    stop: bool,
}
