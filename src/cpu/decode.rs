use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use crate::memory::MemoryIF;

pub fn decode(pc: u16, memory: &impl MemoryIF) -> (Inst, u16) {
    let mut addvance = 1;
    let inst = match memory.read_byte(pc) {
        0x00 => Inst::Nop,
        0x01 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMemory {
        memory: [u8; 0x200],
    }
    impl TestMemory {
        fn new() -> TestMemory {
            TestMemory { memory: [0; 0x200] }
        }
    }
    impl MemoryIF for TestMemory {
        fn read_byte(&self, addr: u16) -> u8 {
            self.memory[addr as usize]
        }
        fn read_word(&self, addr: u16) -> u16 {
            let h = self.memory[addr as usize] as u16;
            let l = self.memory[addr as usize + 1] as u16;
            (h << 8) | l
        }
        fn write_byte(&mut self, addr: u16, val: u8) {
            self.memory[addr as usize] = val;
        }
        fn write_word(&mut self, addr: u16, val: u16) {
            let h = (val >> 8) as u8;
            let l = (val & 0x00ff) as u8;
            self.memory[addr as usize] = h;
            self.memory[addr as usize + 1] = l;
        }
    }

    #[test]
    fn decode_nop() {
        let m = TestMemory::new();
        let pc = 0x0100;
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Nop, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_bc_u16() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x01);
        m.write_word(pc + 1, 0x1234);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(0x1234)), i);
        assert_eq!(3, a);
    }
}
