use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::Cpu;
use crate::memory::MemoryIF;

impl Cpu {
    pub fn decode(&self, memory: &impl MemoryIF) -> (Inst, u16) {
        let pc = self.pc;
        let mut addvance = 1;
        let inst = match memory.read_byte(pc) {
            0x00 => Inst::Nop,
            0x01 => {
                let nn = memory.read_word(pc + 1);
                addvance += 2;
                Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(nn))
            }
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0a => todo!(),
            0x0b => todo!(),
            0x0c => todo!(),
            0x0d => todo!(),
            0x0e => todo!(),
            0x0f => todo!(),
            _ => todo!(),
        };
        (inst, addvance)
    }
}
