use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::{Registers, M};
use crate::memory::MemoryIF;

impl Registers {
    pub fn execute(&mut self, inst: Inst, memory: &impl MemoryIF) -> M {
        let m = match inst {
            Inst::Ld8(dist, src) => self.ld8(dist, src),
            Inst::Nop => 1,
            Inst::Stop => todo!(),
            _ => todo!(),
        };
        m
    }

    fn ld8(&mut self, dist: Arg8, src: Arg8) -> M {
        let m = match (dist, src) {
            (Arg8::Reg(rd), Arg8::Reg(rs)) => {
                let v = self.read_reg8(rs);
                self.write_reg8(rd, v);
                1
            }
            _ => todo!(),
        };
        m
    }
}
