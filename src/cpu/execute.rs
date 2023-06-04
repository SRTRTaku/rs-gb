use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use super::{Registers, M};
use crate::memory::MemoryIF;

impl Registers {
    pub fn execute(&mut self, inst: Inst, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match inst {
            Inst::Ld8(dist, src) => self.ld8(dist, src, memory)?,
            Inst::Nop => 1,
            Inst::Stop => todo!(),
            _ => todo!(),
        };
        Ok(m)
    }

    fn ld8(&mut self, dest: Arg8, src: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match (dest, src) {
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
            (Arg8::IndReg(Reg16::HL), Arg8::Immed(n)) => {
                let hl = self.read_reg16(Reg16::HL);
                memory.write_byte(hl, n);
                3
            }
            (Arg8::Reg(Reg8::A), Arg8::IndReg(r)) if r == Reg16::BC || r == Reg16::DE => {
                let addr = self.read_reg16(r);
                let v = memory.read_byte(addr);
                self.write_reg8(Reg8::A, v);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::Ind(nn)) => {
                let v = memory.read_byte(nn);
                self.write_reg8(Reg8::A, v);
                4
            }
            (Arg8::IndReg(r), Arg8::Reg(Reg8::A)) if r == Reg16::BC || r == Reg16::DE => {
                let v = self.read_reg8(Reg8::A);
                let addr = self.read_reg16(r);
                memory.write_byte(addr, v);
                2
            }
            (Arg8::Ind(nn), Arg8::Reg(Reg8::A)) => {
                let v = self.read_reg8(Reg8::A);
                memory.write_byte(nn, v);
                4
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIo(n)) => {
                let v = memory.read_byte(0xff00 + n as u16);
                self.write_reg8(Reg8::A, v);
                3
            }
            (Arg8::IndIo(n), Arg8::Reg(Reg8::A)) => {
                let v = self.read_reg8(Reg8::A);
                memory.write_byte(0xff00 + n as u16, v);
                3
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIoC) => {
                let c = self.read_reg8(Reg8::C);
                let v = memory.read_byte(0xff00 + c as u16);
                self.write_reg8(Reg8::A, v);
                2
            }
            (Arg8::IndIoC, Arg8::Reg(Reg8::A)) => {
                let c = self.read_reg8(Reg8::C);
                let v = self.read_reg8(Reg8::A);
                memory.write_byte(0xff00 + c as u16, v);
                2
            }
            (Arg8::IndIncHL, Arg8::Reg(Reg8::A)) => {
                let hl = self.read_reg16(Reg16::HL);
                let v = self.read_reg8(Reg8::A);
                memory.write_byte(hl, v);
                self.write_reg16(Reg16::HL, hl + 1);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIncHL) => {
                let hl = self.read_reg16(Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(Reg8::A, v);
                self.write_reg16(Reg16::HL, hl + 1);
                2
            }
            (Arg8::IndDecHL, Arg8::Reg(Reg8::A)) => {
                let hl = self.read_reg16(Reg16::HL);
                let v = self.read_reg8(Reg8::A);
                memory.write_byte(hl, v);
                self.write_reg16(Reg16::HL, hl - 1);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::IndDecHL) => {
                let hl = self.read_reg16(Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(Reg8::A, v);
                self.write_reg16(Reg16::HL, hl - 1);
                2
            }
            (dest, src) => {
                return Err(format!("ld8, Invalid instruction: {:#?}, {:#?}", dest, src))
            }
        };
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMemory {
        memory: [u8; 0x10000],
    }
    impl TestMemory {
        fn new() -> TestMemory {
            TestMemory {
                memory: [0; 0x10000],
            }
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
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_r_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Immed(0x12));
        let m = reg.execute(i, &mut mem).unwrap();
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
        let m = reg.execute(i, &mut mem).unwrap();
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
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_phl_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x100);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Immed(0x12));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_a_pbc() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::BC, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::BC));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_a_pnn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Ind(0x100));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_pbc_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x12);
        reg.write_reg16(Reg16::DE, 0x100);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::DE), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_pnn_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x12);
        let i = Inst::Ld8(Arg8::Ind(0x100), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_a_pff00n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        mem.write_byte(0xff12, 0x34);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIo(0x12));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x34, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_pff00n_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x34);
        let i = Inst::Ld8(Arg8::IndIo(0x12), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x34, mem.read_byte(0xff12));
    }
    #[test]
    fn ld8_a_pff00c() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        mem.write_byte(0xff12, 0x34);
        reg.write_reg8(Reg8::C, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIoC);
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x34, reg.read_reg8(Reg8::A));
    }
    #[test]
    fn ld8_pff00c_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x34);
        reg.write_reg8(Reg8::C, 0x12);
        let i = Inst::Ld8(Arg8::IndIoC, Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x34, mem.read_byte(0xff12));
    }
    #[test]
    fn ld8_phlinc_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x12);
        reg.write_reg16(Reg16::HL, 0x100);
        let i = Inst::Ld8(Arg8::IndIncHL, Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
        assert_eq!(0x101, reg.read_reg16(Reg16::HL));
    }
    #[test]
    fn ld8_a_phldec() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndDecHL);
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(Reg8::A));
        assert_eq!(0xff, reg.read_reg16(Reg16::HL));
    }
}
