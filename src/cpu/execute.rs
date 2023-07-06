use super::inst::{Arg16, Arg8, FlagReg, Inst, Reg16, Reg8};
use super::{Registers, M};
use crate::memory::MemoryIF;

impl Registers {
    pub fn execute(&mut self, inst: Inst, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match inst {
            Inst::Ld8(dist, src) => self.ld8(dist, src, memory)?,
            Inst::Ld16(dist, src) => self.ld16(dist, src, memory)?,
            Inst::Push16(rr) => self.push(rr, memory),
            Inst::Pop16(rr) => self.pop(rr, memory),
            Inst::Add(Arg8::Reg(Reg8::A), x) => self.add_a(x, memory)?,
            Inst::Adc(Arg8::Reg(Reg8::A), x) => self.adc_a(x, memory)?,
            Inst::Sub(Arg8::Reg(Reg8::A), x) => self.sub_a(x, memory)?,
            Inst::Sbc(Arg8::Reg(Reg8::A), x) => self.sbc_a(x, memory)?,
            Inst::And(Arg8::Reg(Reg8::A), x) => self.and_a(x, memory)?,
            Inst::Xor(Arg8::Reg(Reg8::A), x) => self.xor_a(x, memory)?,
            Inst::Or(Arg8::Reg(Reg8::A), x) => self.or_a(x, memory)?,
            Inst::Cp(Arg8::Reg(Reg8::A), x) => self.cp(x, memory)?,
            Inst::Inc(x) => self.inc(x, memory)?,
            Inst::Dec(x) => self.dec(x, memory)?,
            Inst::Daa => self.daa(),
            Inst::Cpl => self.cpl(),
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
            (dest, src) => return Err(format!("ld8, Invalid instruction: {:?}, {:?}", dest, src)),
        };
        Ok(m)
    }

    fn ld16(&mut self, dest: Arg16, src: Arg16, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match (dest, src) {
            (Arg16::Reg(rd), Arg16::Immed(nn)) => {
                self.write_reg16(rd, nn);
                3
            }
            (Arg16::Ind(nn), Arg16::Reg(Reg16::SP)) => {
                let sp = self.read_reg16(Reg16::SP);
                memory.write_word(nn, sp);
                5
            }
            (Arg16::Reg(Reg16::SP), Arg16::Reg(Reg16::HL)) => {
                let v = self.read_reg16(Reg16::HL);
                self.write_reg16(Reg16::SP, v);
                2
            }
            (dest, src) => return Err(format!("ld16, Invalid instruction: {:?}, {:?}", dest, src)),
        };
        Ok(m)
    }

    fn push(&mut self, rr: Reg16, memory: &mut impl MemoryIF) -> M {
        let sp_org = self.read_reg16(Reg16::SP);
        let sp = sp_org - 2;
        self.write_reg16(Reg16::SP, sp);
        let v = self.read_reg16(rr);
        memory.write_word(sp, v);
        4
    }
    fn pop(&mut self, rr: Reg16, memory: &mut impl MemoryIF) -> M {
        let sp_org = self.read_reg16(Reg16::SP);
        let v = memory.read_word(sp_org);
        self.write_reg16(rr, v);
        let sp = sp_org + 2;
        self.write_reg16(Reg16::SP, sp);
        3
    }
    fn add_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("add_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a.wrapping_add(v);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.clear_f(FlagReg::N);
        // H
        let ha = 0x0f & a;
        let hv = 0x0f & v;
        if ha + hv > 0x0f {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if a as u16 + v as u16 > 0x00ff {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn adc_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("adc_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let c = if self.test_f(FlagReg::C) { 0x01 } else { 0x00 };
        let ans = a.wrapping_add(v).wrapping_add(c);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.clear_f(FlagReg::N);
        // H
        let ha = 0x0f & a;
        let hv = 0x0f & v;
        if ha + hv + c > 0x0f {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if a as u16 + v as u16 + c as u16 > 0x00ff {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn sub_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("sub_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a.wrapping_sub(v);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.set_f(FlagReg::N);
        // H
        let ha = 0x0f & a;
        let hv = 0x0f & v;
        if ha < hv {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if a < v {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn sbc_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("sbc_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let c = if self.test_f(FlagReg::C) { 0x01 } else { 0x00 };
        let ans1 = a.wrapping_sub(v);
        let ans2 = ans1.wrapping_sub(c);
        //// set flags
        // Z
        if ans2 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.set_f(FlagReg::N);
        // H
        let ha = 0x0f & a;
        let hv = 0x0f & v;
        let hans1 = 0x0f & ans1;
        if (ha < hv) || (hans1 < c) {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if (a < v) || (ans1 < c) {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        self.write_reg8(Reg8::A, ans2);
        Ok(m)
    }
    fn and_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("and_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a & v;
        //// set flags
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.set_f(FlagReg::H);
        self.clear_f(FlagReg::C);
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn xor_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("xor_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a ^ v;
        //// set flags
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        self.clear_f(FlagReg::C);
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn or_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("or_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a | v;
        //// set flags
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        self.clear_f(FlagReg::C);
        ////
        self.write_reg8(Reg8::A, ans);
        Ok(m)
    }
    fn cp(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("cp, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(Reg8::A);
        let ans = a.wrapping_sub(v);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.set_f(FlagReg::N);
        // H
        let ha = 0x0f & a;
        let hv = 0x0f & v;
        if ha < hv {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if a < v {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        Ok(m)
    }
    fn inc(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x.clone() {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (3, memory.read_byte(hl))
            }
            _ => return Err(format!("inc, Invalid instruction: {:?}", x)),
        };
        let ans = v.wrapping_add(1);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.clear_f(FlagReg::N);
        // H
        let hv = 0x0f & v;
        if hv + 1 > 0x0f {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        ////
        match x {
            Arg8::Reg(r) => self.write_reg8(r, ans),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                memory.write_byte(hl, ans);
            }
            _ => return Err(format!("inc, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn dec(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x.clone() {
            Arg8::Reg(r) => (1, self.read_reg8(r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                (3, memory.read_byte(hl))
            }
            _ => return Err(format!("dec, Invalid instruction: {:?}", x)),
        };
        let ans = v.wrapping_sub(1);
        //// set flags
        // Z
        if ans == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        // N
        self.set_f(FlagReg::N);
        // H
        let hv = 0x0f & v;
        if hv < 1 {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        ////
        match x {
            Arg8::Reg(r) => self.write_reg8(r, ans),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(Reg16::HL);
                memory.write_byte(hl, ans);
            }
            _ => return Err(format!("dec, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn daa(&mut self) -> M {
        let a = self.read_reg8(Reg8::A);
        let a1 = if a & 0x0f > 9 || self.test_f(FlagReg::H) {
            if a as u16 + 6 > 0xff {
                self.set_f(FlagReg::C);
            }
            a.wrapping_add(6)
        } else {
            a
        };
        let a2 = if a & 0xf0 > 0x90 || self.test_f(FlagReg::C) {
            if a1 as u16 + 0x60 > 0xff {
                self.set_f(FlagReg::C);
            }
            a1.wrapping_add(0x60)
        } else {
            a1
        };
        //// set flags
        if a2 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::H);
        ////
        self.write_reg8(Reg8::A, a2);
        let m = 1;
        m
    }
    fn cpl(&mut self) -> M {
        let a = self.read_reg8(Reg8::A);
        let ans = !a;
        //// set flags
        self.set_f(FlagReg::N);
        self.set_f(FlagReg::H);
        ////
        self.write_reg8(Reg8::A, ans);
        let m = 1;
        m
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
            let l = self.memory[addr as usize] as u16;
            let h = self.memory[addr as usize + 1] as u16;
            (h << 8) | l
        }
        fn write_byte(&mut self, addr: u16, val: u8) {
            self.memory[addr as usize] = val;
        }
        fn write_word(&mut self, addr: u16, val: u16) {
            let h = (val >> 8) as u8;
            let l = (val & 0x00ff) as u8;
            self.memory[addr as usize] = l;
            self.memory[addr as usize + 1] = h;
        }
    }

    //
    // 8-bit load instructions
    //
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

    //
    // 16-bit load instructions
    //
    #[test]
    fn ld16_rr_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        let i = Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(0x1234));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x1234, reg.read_reg16(Reg16::BC));
    }
    #[test]
    fn ld16_pnn_sp() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::SP, 0x1234);
        let i = Inst::Ld16(Arg16::Ind(0x100), Arg16::Reg(Reg16::SP));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(5, m);
        assert_eq!(0x1234, mem.read_word(0x100));
    }
    #[test]
    fn ld16_sp_hl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x1234);
        let i = Inst::Ld16(Arg16::Reg(Reg16::SP), Arg16::Reg(Reg16::HL));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x1234, reg.read_reg16(Reg16::SP));
    }
    #[test]
    fn push_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::SP, 0x100);
        reg.write_reg16(Reg16::BC, 0x1234);
        let i = Inst::Push16(Reg16::BC);
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x100 - 2, reg.read_reg16(Reg16::SP));
        assert_eq!(0x1234, mem.read_word(0x100 - 2));
    }
    #[test]
    fn pop_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        mem.write_word(0x100, 0x1234);
        reg.write_reg16(Reg16::SP, 0x100);
        let i = Inst::Pop16(Reg16::DE);
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x100 + 2, reg.read_reg16(Reg16::SP));
        assert_eq!(0x1234, reg.read_reg16(Reg16::DE));
    }
    //
    // 8-bit arithmeric/logic instructions
    //
    #[test]
    fn add_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0xff);
        reg.write_reg8(Reg8::B, 0x01);
        let i = Inst::Add(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x00, reg.read_reg8(Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn adc_a_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0xfe);
        reg.set_f(FlagReg::C);
        let i = Inst::Adc(Arg8::Reg(Reg8::A), Arg8::Immed(0x01));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x00, reg.read_reg8(Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sub_a_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x01);
        reg.write_reg16(Reg16::HL, 0x100);
        mem.write_byte(0x100, 0xff);
        let i = Inst::Sub(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x02, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sbc_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0xff);
        reg.write_reg8(Reg8::C, 0x01);
        reg.set_f(FlagReg::C);
        let i = Inst::Sbc(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::C));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0xfd, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn and_a_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0b101010);
        let i = Inst::And(Arg8::Reg(Reg8::A), Arg8::Immed(0b010101));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x00, reg.read_reg8(Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn xor_a_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0b10101100);
        reg.write_reg16(Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b11001010);
        let i = Inst::Xor(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b01100110, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn or_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0b10101100);
        reg.write_reg8(Reg8::D, 0b11001010);
        let i = Inst::Or(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::D));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b11101110, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn cp_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0x01);
        let i = Inst::Cp(Arg8::Reg(Reg8::A), Arg8::Immed(0xff));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x01, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn inc_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg16(Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x7f);
        let i = Inst::Inc(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x80, mem.read_byte(0x100));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn dec_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::E, 0x10);
        let i = Inst::Dec(Arg8::Reg(Reg8::E));
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x0f, reg.read_reg8(Reg8::E));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn daa() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        // 12 + 34 = 46
        reg.write_reg8(Reg8::A, 0x12);
        let i = Inst::Add(Arg8::Reg(Reg8::A), Arg8::Immed(0x34));
        let _m = reg.execute(i, &mut mem).unwrap();

        let i = Inst::Daa;
        let _m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(0x46, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));

        // 99 + 99 = 198
        reg.write_reg8(Reg8::A, 0x99);
        let i = Inst::Add(Arg8::Reg(Reg8::A), Arg8::Immed(0x99));
        let _m = reg.execute(i, &mut mem).unwrap();

        let i = Inst::Daa;
        let _m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(0x98, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn cpl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();

        reg.write_reg8(Reg8::A, 0xf0);
        let i = Inst::Cpl;
        let m = reg.execute(i, &mut mem).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x0f, reg.read_reg8(Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
}
