use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::Cpu;
use crate::memory::MemoryIF;

impl Cpu {
    pub fn execute(&mut self, inst: Inst) {
        match inst {
            Inst::Nop => {
                self.m = 1;
            }
            Inst::Stop => (),
            Inst::Ld8(dist, src) => self.ld8(dist, src),
            _ => (),
        }
    }

    fn ld8(&mut self, dist: Arg8, src: Arg8) {
        let v = match src {
            Arg8::Reg(r) => match r {
                Reg8::A => self.a,
                _ => panic!(),
            },
            _ => panic!(),
        };
        match dist {
            Arg8::Reg(Reg8::A) => self.a = v,
            _ => panic!(),
        }
    }
}
