use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::{Registers, M};
use crate::memory::MemoryIF;

impl Registers {
    pub fn execute(&mut self, inst: Inst, memory: &mut impl MemoryIF) -> M {
        let m = match inst {
            Inst::Ld8(dist, src) => self.ld8(dist, src, memory),
            Inst::Nop => 1,
            Inst::Stop => todo!(),
            _ => todo!(),
        };
        m
    }

    fn ld8(&mut self, dist: Arg8, src: Arg8, memory: &mut impl MemoryIF) -> M {
        let m = match (dist, src) {
            (Arg8::Reg(rd), Arg8::Reg(rs)) => {
                let v = self.read_reg8(rs);
                self.write_reg8(rd, v);
                1
            }
            (Arg8::Reg(rd), Arg8::Immed(n)) => {
                self.write_reg8(rd, n);
                2
            }
            (Arg8::Reg(rd), Arg8::IndReg(Reg16::HL)) => {
                let hl = self.read_reg16(Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(rd, v);
                2
            }
            (Arg8::IndReg(Reg16::HL), Arg8::Reg(rs)) => {
                let v = self.read_reg8(rs);
                let hl = self.read_reg16(Reg16::HL);
                memory.write_byte(hl, v);
                2
            }
            _ => todo!(),
        };
        m
    }
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
    fn ld8_r_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::B, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem);
        assert_eq!(1, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_r_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Immed(0x12));
        let m = reg.execute(i, &mut mem);
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_r_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem);
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_phl_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x100);
        reg.write_reg8(Reg8::A, 0x12);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem);
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
}
