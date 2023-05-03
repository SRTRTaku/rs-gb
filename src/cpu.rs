mod decode;
mod decode_prefix_cb;
mod execute;
mod inst;
use crate::memory::MemoryIF;
use inst::{Reg16, Reg8};

pub struct Cpu {
    clock_m: usize,
    // t = 4m
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
    // Clock for last instr
    m: usize,
    // t = 4m
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            clock_m: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            m: 0,
        }
    }
    pub fn run(&mut self, memory: &impl MemoryIF) -> Result<(), String> {
        let (inst, addvance) = decode::decode(self.pc, memory)?;
        self.pc += addvance;
        self.execute(inst);
        Ok(())
    }
    fn read_reg8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
            Reg8::F => self.f,
        }
    }
    fn write_reg8(&mut self, r: Reg8, v: u8) {
        match r {
            Reg8::A => self.a = v,
            Reg8::B => self.b = v,
            Reg8::C => self.c = v,
            Reg8::D => self.d = v,
            Reg8::E => self.e = v,
            Reg8::H => self.h = v,
            Reg8::L => self.l = v,
            Reg8::F => self.f = v,
        }
    }
    fn read_reg16(&self, r: Reg16) -> u16 {
        match r {
            // Reg16::AF => xx,
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
            // Reg16::PC => xx,
            Reg16::SP => self.sp,
            _ => panic!(),
        }
    }
    fn write_reg16(&mut self, r: Reg16, v: u16) {
        let v0 = (v >> 8) as u8;
        let v1 = (v & 0x00ff) as u8;
        match r {
            // Reg16::AF => xxx,
            Reg16::BC => {
                self.b = v0;
                self.c = v1;
            }
            Reg16::DE => {
                self.b = v0;
                self.c = v1;
            }
            Reg16::HL => {
                self.b = v0;
                self.c = v1;
            }
            // Reg16::CP
            Reg16::SP => self.sp = v,
            _ => panic!(),
        }
    }
}
