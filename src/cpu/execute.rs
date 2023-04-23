use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::Cpu;
use crate::memory::MemoryIF;

impl Cpu {
    pub fn execute(&mut self, inst: Inst) {
        match inst {
            Inst::Ld8(dist, src) => self.ld8(dist, src),
            Inst::Nop => {
                self.m = 1;
            }
            Inst::Stop => (),
            _ => (),
        }
    }

    fn ld8(&mut self, dist: Arg8, src: Arg8) {
        match (dist, src) {
            (Arg8::Reg(rd), Arg8::Reg(rs)) => {
                self.m = 1;
                let v = self.read_reg8(rs);
                self.write_reg8(rd, v);
            }
            _ => todo!(),
        }
    }
}
