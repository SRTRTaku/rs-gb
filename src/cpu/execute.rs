use super::inst::{Arg16, Arg8, FlagReg, Inst, JpFlag, Reg16, Reg8};
use super::{Registers, M};
use crate::memory::MemoryIF;

impl Registers {
    pub fn execute(
        &mut self,
        inst: Inst,
        memory: &mut impl MemoryIF,
        ime: &mut bool,
    ) -> Result<M, String> {
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
            Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(rr)) => self.add16_hl(rr),
            Inst::Inc16(Arg16::Reg(rr)) => self.inc16_rr(rr),
            Inst::Dec16(Arg16::Reg(rr)) => self.dec16_rr(rr),
            Inst::Add16SP(dd) => self.add16_sp_dd(dd),
            Inst::Ld16HLSP(dd) => self.ld16_hl_sp_dd(dd),
            Inst::Rlca => self.rlca(),
            Inst::Rla => self.rla(),
            Inst::Rrca => self.rrca(),
            Inst::Rra => self.rra(),
            Inst::Rlc(x) => self.rlc(&x, memory)?,
            Inst::Rl(x) => self.rl(&x, memory)?,
            Inst::Rrc(x) => self.rrc(&x, memory)?,
            Inst::Rr(x) => self.rr(&x, memory)?,
            Inst::Sla(x) => self.sla(&x, memory)?,
            Inst::Swap(x) => self.swap(&x, memory)?,
            Inst::Sra(x) => self.sra(&x, memory)?,
            Inst::Srl(x) => self.srl(&x, memory)?,
            Inst::Bit(n, x) => self.bit(n, &x, memory)?,
            Inst::Set(n, x) => self.set(n, &x, memory)?,
            Inst::Res(n, x) => self.res(n, &x, memory)?,
            Inst::Ccf => self.ccf(),
            Inst::Scf => self.scf(),
            Inst::Nop => 1,
            Inst::Halt => todo!(),
            Inst::Stop => todo!(),
            Inst::Di => self.di(ime),
            Inst::Ei => self.ei(ime),
            Inst::Jp(nn) => self.jp_nn(nn),
            Inst::JpHL => self.jp_hl(),
            Inst::Jpf(f, nn) => self.jp_f_nn(f, nn),
            Inst::Jr(dd) => self.jr_dd(dd),
            Inst::Jrf(f, dd) => self.jr_f_dd(f, dd),
            Inst::Call(nn) => self.call_nn(nn, memory),
            Inst::Callf(f, nn) => self.call_f_nn(f, nn, memory),
            Inst::Ret => self.ret(memory),
            Inst::Retf(f) => self.ret_f(f, memory),
            Inst::Reti => self.reti(memory, ime),
            Inst::Rst(n) => self.rst_n(n, memory),
            i => return Err(format!("execute, Invalid instruction: {:?}", i)),
        };
        Ok(m)
    }

    fn ld8(&mut self, dest: Arg8, src: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match (dest, src) {
            (Arg8::Reg(rd), Arg8::Reg(rs)) => {
                let v = self.read_reg8(&rs);
                self.write_reg8(&rd, v);
                1
            }
            (Arg8::Reg(rd), Arg8::Immed(n)) => {
                self.write_reg8(&rd, n);
                2
            }
            (Arg8::Reg(rd), Arg8::IndReg(Reg16::HL)) => {
                let hl = self.read_reg16(&Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(&rd, v);
                2
            }
            (Arg8::IndReg(Reg16::HL), Arg8::Reg(rs)) => {
                let v = self.read_reg8(&rs);
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v);
                2
            }
            (Arg8::IndReg(Reg16::HL), Arg8::Immed(n)) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, n);
                3
            }
            (Arg8::Reg(Reg8::A), Arg8::IndReg(r)) if r == Reg16::BC || r == Reg16::DE => {
                let addr = self.read_reg16(&r);
                let v = memory.read_byte(addr);
                self.write_reg8(&Reg8::A, v);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::Ind(nn)) => {
                let v = memory.read_byte(nn);
                self.write_reg8(&Reg8::A, v);
                4
            }
            (Arg8::IndReg(r), Arg8::Reg(Reg8::A)) if r == Reg16::BC || r == Reg16::DE => {
                let v = self.read_reg8(&Reg8::A);
                let addr = self.read_reg16(&r);
                memory.write_byte(addr, v);
                2
            }
            (Arg8::Ind(nn), Arg8::Reg(Reg8::A)) => {
                let v = self.read_reg8(&Reg8::A);
                memory.write_byte(nn, v);
                4
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIo(n)) => {
                let v = memory.read_byte(0xff00 + n as u16);
                self.write_reg8(&Reg8::A, v);
                3
            }
            (Arg8::IndIo(n), Arg8::Reg(Reg8::A)) => {
                let v = self.read_reg8(&Reg8::A);
                memory.write_byte(0xff00 + n as u16, v);
                3
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIoC) => {
                let c = self.read_reg8(&Reg8::C);
                let v = memory.read_byte(0xff00 + c as u16);
                self.write_reg8(&Reg8::A, v);
                2
            }
            (Arg8::IndIoC, Arg8::Reg(Reg8::A)) => {
                let c = self.read_reg8(&Reg8::C);
                let v = self.read_reg8(&Reg8::A);
                memory.write_byte(0xff00 + c as u16, v);
                2
            }
            (Arg8::IndIncHL, Arg8::Reg(Reg8::A)) => {
                let hl = self.read_reg16(&Reg16::HL);
                let v = self.read_reg8(&Reg8::A);
                memory.write_byte(hl, v);
                self.write_reg16(&Reg16::HL, hl + 1);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::IndIncHL) => {
                let hl = self.read_reg16(&Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(&Reg8::A, v);
                self.write_reg16(&Reg16::HL, hl + 1);
                2
            }
            (Arg8::IndDecHL, Arg8::Reg(Reg8::A)) => {
                let hl = self.read_reg16(&Reg16::HL);
                let v = self.read_reg8(&Reg8::A);
                memory.write_byte(hl, v);
                self.write_reg16(&Reg16::HL, hl - 1);
                2
            }
            (Arg8::Reg(Reg8::A), Arg8::IndDecHL) => {
                let hl = self.read_reg16(&Reg16::HL);
                let v = memory.read_byte(hl);
                self.write_reg8(&Reg8::A, v);
                self.write_reg16(&Reg16::HL, hl - 1);
                2
            }
            (dest, src) => return Err(format!("ld8, Invalid instruction: {:?}, {:?}", dest, src)),
        };
        Ok(m)
    }

    fn ld16(&mut self, dest: Arg16, src: Arg16, memory: &mut impl MemoryIF) -> Result<M, String> {
        let m = match (dest, src) {
            (Arg16::Reg(rd), Arg16::Immed(nn)) => {
                self.write_reg16(&rd, nn);
                3
            }
            (Arg16::Ind(nn), Arg16::Reg(Reg16::SP)) => {
                let sp = self.read_reg16(&Reg16::SP);
                memory.write_word(nn, sp);
                5
            }
            (Arg16::Reg(Reg16::SP), Arg16::Reg(Reg16::HL)) => {
                let v = self.read_reg16(&Reg16::HL);
                self.write_reg16(&Reg16::SP, v);
                2
            }
            (dest, src) => return Err(format!("ld16, Invalid instruction: {:?}, {:?}", dest, src)),
        };
        Ok(m)
    }

    fn push(&mut self, rr: Reg16, memory: &mut impl MemoryIF) -> M {
        let sp_org = self.read_reg16(&Reg16::SP);
        let sp = sp_org - 2;
        self.write_reg16(&Reg16::SP, sp);
        let v = self.read_reg16(&rr);
        memory.write_word(sp, v);
        4
    }
    fn pop(&mut self, rr: Reg16, memory: &mut impl MemoryIF) -> M {
        let sp_org = self.read_reg16(&Reg16::SP);
        let v = memory.read_word(sp_org);
        self.write_reg16(&rr, v);
        let sp = sp_org + 2;
        self.write_reg16(&Reg16::SP, sp);
        3
    }
    fn add_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("add_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn adc_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("adc_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn sub_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("sub_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn sbc_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("sbc_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans2);
        Ok(m)
    }
    fn and_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("and_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn xor_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("xor_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn or_a(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("or_a, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
        self.write_reg8(&Reg8::A, ans);
        Ok(m)
    }
    fn cp(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::Immed(n) => (2, n),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (2, memory.read_byte(hl))
            }
            _ => return Err(format!("cp, Invalid instruction: {:?}", x)),
        };
        let a = self.read_reg8(&Reg8::A);
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
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
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
            Arg8::Reg(r) => self.write_reg8(&r, ans),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, ans);
            }
            _ => return Err(format!("inc, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn dec(&mut self, x: Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x.clone() {
            Arg8::Reg(r) => (1, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
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
            Arg8::Reg(r) => self.write_reg8(&r, ans),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, ans);
            }
            _ => return Err(format!("dec, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn daa(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let a3 = if self.test_f(FlagReg::N) {
            let a1 = if self.test_f(FlagReg::C) {
                a.wrapping_sub(0x60)
            } else {
                a
            };
            let a2 = if self.test_f(FlagReg::H) {
                a1.wrapping_sub(0x6)
            } else {
                a1
            };
            a2
        } else {
            let a1 = if a > 0x99 || self.test_f(FlagReg::C) {
                self.set_f(FlagReg::C);
                a.wrapping_add(0x60)
            } else {
                a
            };
            let a2 = if a1 & 0x0f > 0x09 || self.test_f(FlagReg::H) {
                a1.wrapping_add(0x6)
            } else {
                a1
            };
            a2
        };
        //// set flags
        if a3 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::H);
        ////
        self.write_reg8(&Reg8::A, a3);
        let m = 1;
        m
    }
    fn cpl(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let ans = !a;
        //// set flags
        self.set_f(FlagReg::N);
        self.set_f(FlagReg::H);
        ////
        self.write_reg8(&Reg8::A, ans);
        let m = 1;
        m
    }
    fn add16_hl(&mut self, rr: Reg16) -> M {
        let hl = self.read_reg16(&Reg16::HL);
        let v = self.read_reg16(&rr);
        let ans = hl.wrapping_add(v);
        //// set flags
        // N
        self.clear_f(FlagReg::N);
        // H
        let hhl = 0x0fff & hl;
        let hv = 0x0fff & v;
        if hhl + hv > 0x0fff {
            self.set_f(FlagReg::H);
        } else {
            self.clear_f(FlagReg::H);
        }
        // C
        if hl as u32 + v as u32 > 0xffff {
            self.set_f(FlagReg::C);
        } else {
            self.clear_f(FlagReg::C);
        }
        ////
        self.write_reg16(&Reg16::HL, ans);
        let m = 2;
        m
    }
    fn inc16_rr(&mut self, rr: Reg16) -> M {
        let v = self.read_reg16(&rr);
        let ans = v.wrapping_add(1);
        self.write_reg16(&rr, ans);
        let m = 2;
        m
    }
    fn dec16_rr(&mut self, rr: Reg16) -> M {
        let v = self.read_reg16(&rr);
        let ans = v.wrapping_sub(1);
        self.write_reg16(&rr, ans);
        let m = 2;
        m
    }
    fn add16_sp_dd(&mut self, dd: i8) -> M {
        let sp = self.read_reg16(&Reg16::SP);
        let ans = if dd > 0 {
            sp.wrapping_add(dd as u16)
        } else {
            sp.wrapping_sub((-dd) as u16)
        };
        self.write_reg16(&Reg16::SP, ans);
        let m = 4;
        m
    }
    fn ld16_hl_sp_dd(&mut self, dd: i8) -> M {
        let sp = self.read_reg16(&Reg16::SP);
        let ans = if dd > 0 {
            sp.wrapping_add(dd as u16)
        } else {
            sp.wrapping_sub((-dd) as u16)
        };
        self.write_reg16(&Reg16::HL, ans);
        let m = 3;
        m
    }
    fn rlca(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let (a1, c) = rot(a, Direction::Left);
        if a1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        self.write_reg8(&Reg8::A, a1);
        1 // m
    }
    fn rla(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let c = self.test_f(FlagReg::C);
        let (a1, c1) = rot_through_carry(a, c, Direction::Left);
        if a1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c1 {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        self.write_reg8(&Reg8::A, a1);
        1 // m
    }
    fn rrca(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let (a1, c) = rot(a, Direction::Right);
        if a1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        self.write_reg8(&Reg8::A, a1);
        1 // m
    }
    fn rra(&mut self) -> M {
        let a = self.read_reg8(&Reg8::A);
        let c = self.test_f(FlagReg::C);
        let (a1, c1) = rot_through_carry(a, c, Direction::Right);
        if a1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c1 {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        self.write_reg8(&Reg8::A, a1);
        1 // m
    }
    fn rlc(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("rlc, Invalid instruction: {:?}", x)),
        };
        let (v1, c) = rot(v, Direction::Left);
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("rlc, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn rl(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("rl, Invalid instruction: {:?}", x)),
        };
        let c = self.test_f(FlagReg::C);
        let (v1, c1) = rot_through_carry(v, c, Direction::Left);
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c1 {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("rl, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn rrc(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("rlr, Invalid instruction: {:?}", x)),
        };
        let (v1, c) = rot(v, Direction::Right);
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("rrc, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn rr(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("rr, Invalid instruction: {:?}", x)),
        };
        let c = self.test_f(FlagReg::C);
        let (v1, c1) = rot_through_carry(v, c, Direction::Right);
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c1 {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("rr, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn sla(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("sla, Invalid instruction: {:?}", x)),
        };
        let v1 = v << 1;
        let c = v & 0x80 != 0;
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("sla, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn swap(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("swap, Invalid instruction: {:?}", x)),
        };
        let v1 = ((v & 0xf0) >> 4) | ((v & 0x0f) << 4);
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        self.clear_f(FlagReg::C);
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("swap, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn sra(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("sra, Invalid instruction: {:?}", x)),
        };
        let v1 = (v >> 1) | (v & 0x80);
        let c = v & 0x01 != 0;
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("sra, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn srl(&mut self, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("srl, Invalid instruction: {:?}", x)),
        };
        let v1 = v >> 1;
        let c = v & 0x01 != 0;
        //
        if v1 == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if c {
            self.set_f(FlagReg::C)
        } else {
            self.clear_f(FlagReg::C)
        };
        //
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("srl, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn bit(&mut self, n: u8, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (3, memory.read_byte(hl))
            }
            _ => return Err(format!("bit, Invalid instruction: {:?}", x)),
        };
        if v & (1 << n) == 0 {
            self.set_f(FlagReg::Z);
        } else {
            self.clear_f(FlagReg::Z);
        }
        self.clear_f(FlagReg::N);
        self.set_f(FlagReg::H);
        Ok(m)
    }
    fn set(&mut self, n: u8, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("set, Invalid instruction: {:?}", x)),
        };
        let v1 = v | (1 << n);
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("set, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn res(&mut self, n: u8, x: &Arg8, memory: &mut impl MemoryIF) -> Result<M, String> {
        let (m, v) = match x {
            Arg8::Reg(r) => (2, self.read_reg8(&r)),
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                (4, memory.read_byte(hl))
            }
            _ => return Err(format!("res, Invalid instruction: {:?}", x)),
        };
        let v1 = v & !(1 << n);
        match x {
            Arg8::Reg(r) => {
                self.write_reg8(&r, v1);
            }
            Arg8::IndReg(Reg16::HL) => {
                let hl = self.read_reg16(&Reg16::HL);
                memory.write_byte(hl, v1);
            }
            _ => return Err(format!("res, Invalid instruction: {:?}", x)),
        }
        Ok(m)
    }
    fn ccf(&mut self) -> M {
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        if self.test_f(FlagReg::C) {
            self.clear_f(FlagReg::C);
        } else {
            self.set_f(FlagReg::C);
        };
        1
    }
    fn scf(&mut self) -> M {
        self.clear_f(FlagReg::N);
        self.clear_f(FlagReg::H);
        self.set_f(FlagReg::C);
        1
    }
    fn di(&mut self, ime: &mut bool) -> M {
        *ime = false;
        1
    }
    fn ei(&mut self, ime: &mut bool) -> M {
        *ime = true;
        1
    }
    fn jp_nn(&mut self, nn: u16) -> M {
        self.write_reg16(&Reg16::PC, nn);
        4
    }
    fn jp_hl(&mut self) -> M {
        let hl = self.read_reg16(&Reg16::HL);
        self.write_reg16(&Reg16::PC, hl);
        1
    }
    fn jp_f_nn(&mut self, f: JpFlag, nn: u16) -> M {
        let z = self.test_f(FlagReg::Z);
        let c = self.test_f(FlagReg::C);
        let branch = match f {
            JpFlag::Nz => !z,
            JpFlag::Z => z,
            JpFlag::Nc => !c,
            JpFlag::C => c,
        };
        if branch {
            self.write_reg16(&Reg16::PC, nn);
            4
        } else {
            3
        }
    }
    fn jr_dd(&mut self, dd: i8) -> M {
        let pc = self.read_reg16(&Reg16::PC);
        let pc1 = if dd >= 0 {
            pc.wrapping_add(dd as u16)
        } else {
            pc.wrapping_sub((-dd) as u16)
        };
        self.write_reg16(&Reg16::PC, pc1);
        3
    }
    fn jr_f_dd(&mut self, f: JpFlag, dd: i8) -> M {
        let z = self.test_f(FlagReg::Z);
        let c = self.test_f(FlagReg::C);
        let branch = match f {
            JpFlag::Nz => !z,
            JpFlag::Z => z,
            JpFlag::Nc => !c,
            JpFlag::C => c,
        };
        if branch {
            let pc = self.read_reg16(&Reg16::PC);
            let pc1 = if dd >= 0 {
                pc.wrapping_add(dd as u16)
            } else {
                pc.wrapping_sub((-dd) as u16)
            };
            self.write_reg16(&Reg16::PC, pc1);
            3
        } else {
            2
        }
    }
    fn call_nn(&mut self, nn: u16, memory: &mut impl MemoryIF) -> M {
        let pc = self.read_reg16(&Reg16::PC);
        let sp = self.read_reg16(&Reg16::SP);
        self.write_reg16(&Reg16::SP, sp - 2);
        memory.write_word(sp - 2, pc);
        self.write_reg16(&Reg16::PC, nn);
        6
    }
    fn call_f_nn(&mut self, f: JpFlag, nn: u16, memory: &mut impl MemoryIF) -> M {
        let z = self.test_f(FlagReg::Z);
        let c = self.test_f(FlagReg::C);
        let branch = match f {
            JpFlag::Nz => !z,
            JpFlag::Z => z,
            JpFlag::Nc => !c,
            JpFlag::C => c,
        };
        if branch {
            let pc = self.read_reg16(&Reg16::PC);
            let sp = self.read_reg16(&Reg16::SP);
            self.write_reg16(&Reg16::SP, sp - 2);
            memory.write_word(sp - 2, pc);
            self.write_reg16(&Reg16::PC, nn);
            6
        } else {
            3
        }
    }
    fn ret(&mut self, memory: &mut impl MemoryIF) -> M {
        let sp = self.read_reg16(&Reg16::SP);
        let pc = memory.read_word(sp);
        self.write_reg16(&Reg16::SP, sp + 2);
        self.write_reg16(&Reg16::PC, pc);
        4
    }
    fn ret_f(&mut self, f: JpFlag, memory: &mut impl MemoryIF) -> M {
        let z = self.test_f(FlagReg::Z);
        let c = self.test_f(FlagReg::C);
        let branch = match f {
            JpFlag::Nz => !z,
            JpFlag::Z => z,
            JpFlag::Nc => !c,
            JpFlag::C => c,
        };
        if branch {
            let sp = self.read_reg16(&Reg16::SP);
            let pc = memory.read_word(sp);
            self.write_reg16(&Reg16::SP, sp + 2);
            self.write_reg16(&Reg16::PC, pc);
            5
        } else {
            2
        }
    }
    fn reti(&mut self, memory: &mut impl MemoryIF, ime: &mut bool) -> M {
        let sp = self.read_reg16(&Reg16::SP);
        let pc = memory.read_word(sp);
        self.write_reg16(&Reg16::SP, sp + 2);
        self.write_reg16(&Reg16::PC, pc);
        *ime = true;
        4
    }
    fn rst_n(&mut self, n: u8, memory: &mut impl MemoryIF) -> M {
        let pc = self.read_reg16(&Reg16::PC);
        let sp = self.read_reg16(&Reg16::SP);
        self.write_reg16(&Reg16::SP, sp - 2);
        memory.write_word(sp - 2, pc);
        self.write_reg16(&Reg16::PC, n as u16);
        4
    }
}

// utils
enum Direction {
    Left,
    Right,
}
type Carry = bool;

fn rot(v: u8, d: Direction) -> (u8, Carry) {
    match d {
        Direction::Left => {
            let v1 = v << 1;
            let v2 = if v & 0x80 != 0 { v1 | 0x01 } else { v1 };
            let c = v & 0x80 != 0;
            (v2, c)
        }
        Direction::Right => {
            let v1 = v >> 1;
            let v2 = if v & 0x01 != 0 { v1 | 0x80 } else { v1 };
            let c = v & 0x01 != 0;
            (v2, c)
        }
    }
}
fn rot_through_carry(v: u8, c: Carry, d: Direction) -> (u8, Carry) {
    match d {
        Direction::Left => {
            let v1 = v << 1;
            let v2 = if c { v1 | 0x01 } else { v1 };
            let c1 = v & 0x80 != 0;
            (v2, c1)
        }
        Direction::Right => {
            let v1 = v >> 1;
            let v2 = if c { v1 | 0x80 } else { v1 };
            let c1 = v & 0x01 != 0;
            (v2, c1)
        }
    }
}

// test
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
        fn write_byte(&mut self, addr: u16, val: u8) {
            self.memory[addr as usize] = val;
        }
    }

    //
    // 8-bit load instructions
    //
    #[test]
    fn ld8_r_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::B, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_r_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Immed(0x12));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_r_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_phl_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        reg.write_reg8(&Reg8::A, 0x12);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_phl_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Immed(0x12));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_a_pbc() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::BC, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::BC));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_a_pnn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Ind(0x100));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_pbc_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x12);
        reg.write_reg16(&Reg16::DE, 0x100);
        let i = Inst::Ld8(Arg8::IndReg(Reg16::DE), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_pnn_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x12);
        let i = Inst::Ld8(Arg8::Ind(0x100), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x12, mem.read_byte(0x100));
    }
    #[test]
    fn ld8_a_pff00n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        mem.write_byte(0xff12, 0x34);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIo(0x12));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x34, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_pff00n_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x34);
        let i = Inst::Ld8(Arg8::IndIo(0x12), Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x34, mem.read_byte(0xff12));
    }
    #[test]
    fn ld8_a_pff00c() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        mem.write_byte(0xff12, 0x34);
        reg.write_reg8(&Reg8::C, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIoC);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x34, reg.read_reg8(&Reg8::A));
    }
    #[test]
    fn ld8_pff00c_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x34);
        reg.write_reg8(&Reg8::C, 0x12);
        let i = Inst::Ld8(Arg8::IndIoC, Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x34, mem.read_byte(0xff12));
    }
    #[test]
    fn ld8_phlinc_a() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x12);
        reg.write_reg16(&Reg16::HL, 0x100);
        let i = Inst::Ld8(Arg8::IndIncHL, Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, mem.read_byte(0x100));
        assert_eq!(0x101, reg.read_reg16(&Reg16::HL));
    }
    #[test]
    fn ld8_a_phldec() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x12);
        let i = Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndDecHL);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x12, reg.read_reg8(&Reg8::A));
        assert_eq!(0xff, reg.read_reg16(&Reg16::HL));
    }

    //
    // 16-bit load instructions
    //
    #[test]
    fn ld16_rr_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        let i = Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(0x1234));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x1234, reg.read_reg16(&Reg16::BC));
    }
    #[test]
    fn ld16_pnn_sp() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::SP, 0x1234);
        let i = Inst::Ld16(Arg16::Ind(0x100), Arg16::Reg(Reg16::SP));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(5, m);
        assert_eq!(0x1234, mem.read_word(0x100));
    }
    #[test]
    fn ld16_sp_hl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x1234);
        let i = Inst::Ld16(Arg16::Reg(Reg16::SP), Arg16::Reg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x1234, reg.read_reg16(&Reg16::SP));
    }
    #[test]
    fn push_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::SP, 0x100);
        reg.write_reg16(&Reg16::BC, 0x1234);
        let i = Inst::Push16(Reg16::BC);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x100 - 2, reg.read_reg16(&Reg16::SP));
        assert_eq!(0x1234, mem.read_word(0x100 - 2));
    }
    #[test]
    fn pop_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        mem.write_word(0x100, 0x1234);
        reg.write_reg16(&Reg16::SP, 0x100);
        let i = Inst::Pop16(Reg16::DE);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x100 + 2, reg.read_reg16(&Reg16::SP));
        assert_eq!(0x1234, reg.read_reg16(&Reg16::DE));
    }
    //
    // 8-bit arithmeric/logic instructions
    //
    #[test]
    fn add_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0xff);
        reg.write_reg8(&Reg8::B, 0x01);
        let i = Inst::Add(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x00, reg.read_reg8(&Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn adc_a_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0xfe);
        reg.set_f(FlagReg::C);
        let i = Inst::Adc(Arg8::Reg(Reg8::A), Arg8::Immed(0x01));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x00, reg.read_reg8(&Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sub_a_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x01);
        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0xff);
        let i = Inst::Sub(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x02, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sbc_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0xff);
        reg.write_reg8(&Reg8::C, 0x01);
        reg.set_f(FlagReg::C);
        let i = Inst::Sbc(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::C));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0xfd, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn and_a_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b101010);
        let i = Inst::And(Arg8::Reg(Reg8::A), Arg8::Immed(0b010101));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x00, reg.read_reg8(&Reg8::A));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn xor_a_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10101100);
        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b11001010);
        let i = Inst::Xor(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b01100110, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn or_a_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10101100);
        reg.write_reg8(&Reg8::D, 0b11001010);
        let i = Inst::Or(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::D));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b11101110, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn cp_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0x01);
        let i = Inst::Cp(Arg8::Reg(Reg8::A), Arg8::Immed(0xff));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x01, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn inc_phl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x7f);
        let i = Inst::Inc(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
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
        let mut ime = false;

        reg.write_reg8(&Reg8::E, 0x10);
        let i = Inst::Dec(Arg8::Reg(Reg8::E));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x0f, reg.read_reg8(&Reg8::E));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn daa() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        for m in 0..=99 {
            for n in m..=99 {
                let m_hex = 16 * (m / 10) + (m % 10);
                let n_hex = 16 * (n / 10) + (n % 10);

                reg.write_reg16(&Reg16::AF, 0);
                reg.write_reg8(&Reg8::A, m_hex);
                let i = Inst::Add(Arg8::Reg(Reg8::A), Arg8::Immed(n_hex));
                let _m = reg.execute(i, &mut mem, &mut ime).unwrap();

                let i = Inst::Daa;
                let _m = reg.execute(i, &mut mem, &mut ime).unwrap();

                let ans = m + n;
                let ans_hex = if ans >= 100 {
                    let ans = ans - 100;
                    16 * (ans / 10) + (ans % 10)
                } else {
                    16 * (ans / 10) + (ans % 10)
                };
                let z_des = ans_hex == 0;
                let c_des = ans >= 100;

                assert_eq!(ans_hex, reg.read_reg8(&Reg8::A));
                assert_eq!(z_des, reg.test_f(FlagReg::Z));
                assert_eq!(false, reg.test_f(FlagReg::H));
                assert_eq!(c_des, reg.test_f(FlagReg::C));
            }
        }

        for m in 0..=99 {
            for n in m..=99 {
                let m_hex = 16 * (m / 10) + (m % 10);
                let n_hex = 16 * (n / 10) + (n % 10);
                println!("{}, {}", m, n);

                reg.write_reg16(&Reg16::AF, 0);
                reg.write_reg8(&Reg8::A, n_hex);
                let i = Inst::Sub(Arg8::Reg(Reg8::A), Arg8::Immed(m_hex));
                let _m = reg.execute(i, &mut mem, &mut ime).unwrap();
                println!("{}", reg);

                let i = Inst::Daa;
                let _m = reg.execute(i, &mut mem, &mut ime).unwrap();
                println!("{}", reg);

                let ans = n - m;
                let ans_hex = if ans >= 100 {
                    let ans = ans - 100;
                    16 * (ans / 10) + (ans % 10)
                } else {
                    16 * (ans / 10) + (ans % 10)
                };
                let z_des = ans_hex == 0;
                let c_des = ans >= 100;

                assert_eq!(ans_hex, reg.read_reg8(&Reg8::A));
                assert_eq!(z_des, reg.test_f(FlagReg::Z));
                assert_eq!(false, reg.test_f(FlagReg::H));
                assert_eq!(c_des, reg.test_f(FlagReg::C));
            }
        }
    }
    #[test]
    fn cpl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0xf0);
        let i = Inst::Cpl;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x0f, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(true, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    //
    // 16-bit arithmetic/logic instructions
    //
    #[test]
    fn add_hl_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0xffff);
        reg.write_reg16(&Reg16::BC, 0x0001);
        let i = Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::BC));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x0000, reg.read_reg16(&Reg16::HL));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn inc_rr() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::DE, 0x000f);
        let i = Inst::Inc16(Arg16::Reg(Reg16::DE));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x0010, reg.read_reg16(&Reg16::DE));
    }
    #[test]
    fn add_sp_dd() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::SP, 0x0100);
        let i = Inst::Add16SP(-1);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x00ff, reg.read_reg16(&Reg16::SP));
    }
    #[test]
    fn ld_hl_sp_dd() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x0000);
        reg.write_reg16(&Reg16::SP, 0x0100);
        let i = Inst::Ld16HLSP(-1);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x00ff, reg.read_reg16(&Reg16::HL));
    }
    //
    // rotate & shift instructions
    //
    #[test]
    fn rlca() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10011001);
        let i = Inst::Rlca;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b00110011, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rla() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b00011001);
        reg.set_f(FlagReg::C);
        let i = Inst::Rla;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b00110011, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rrca() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10011001);
        let i = Inst::Rrca;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b11001100, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rra() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10011000);
        reg.set_f(FlagReg::C);
        let i = Inst::Rra;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0b11001100, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rlc_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b10000000);
        let i = Inst::Rlc(Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b00000001, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rl_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b10000000);
        let i = Inst::Rl(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b00000000, mem.read_byte(0x100));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));

        // ---
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;
        reg.set_f(FlagReg::C);

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b00000000);
        let i = Inst::Rl(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b00000001, mem.read_byte(0x100));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rrc_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::B, 0b00000001);
        let i = Inst::Rrc(Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b10000000, reg.read_reg8(&Reg8::B));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn rr_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b00000001);
        let i = Inst::Rr(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b00000000, mem.read_byte(0x100));
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));

        // ---
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;
        reg.set_f(FlagReg::C);

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b00000000);
        let i = Inst::Rr(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b10000000, mem.read_byte(0x100));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sla_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::C, 0b10011001);
        let i = Inst::Sla(Arg8::Reg(Reg8::C));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b00110010, reg.read_reg8(&Reg8::C));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn swap_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b10100101);
        let i = Inst::Swap(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b01011010, mem.read_byte(0x100));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
    }
    #[test]
    fn sra_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0b10011001);
        let i = Inst::Sra(Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b11001100, mem.read_byte(0x100));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn srl_r() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::D, 0b10011001);
        let i = Inst::Srl(Arg8::Reg(Reg8::D));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b01001100, reg.read_reg8(&Reg8::D));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    //
    // single-bit operation instructions
    //
    #[test]
    fn bit() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::A, 0b00100000);
        let i = Inst::Bit(5, Arg8::Reg(Reg8::A));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b00100000, reg.read_reg8(&Reg8::A));
        assert_eq!(false, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));

        let i = Inst::Bit(4, Arg8::Reg(Reg8::A));
        reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(true, reg.test_f(FlagReg::Z));
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(true, reg.test_f(FlagReg::H));
    }
    #[test]
    fn set() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x100);
        mem.write_byte(0x100, 0x00);
        let i = Inst::Set(3, Arg8::IndReg(Reg16::HL));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0b00001000, mem.read_byte(0x100));
    }
    #[test]
    fn res() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg8(&Reg8::B, 0xff);
        let i = Inst::Res(2, Arg8::Reg(Reg8::B));
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0b11111011, reg.read_reg8(&Reg8::B));
    }
    //
    // cpu control instructions
    //
    #[test]
    fn ccf() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.set_f(FlagReg::C);
        let i = Inst::Ccf;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(false, reg.test_f(FlagReg::C));
        let i = Inst::Ccf;
        let _ = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn scf() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        let i = Inst::Scf;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(false, reg.test_f(FlagReg::N));
        assert_eq!(false, reg.test_f(FlagReg::H));
        assert_eq!(true, reg.test_f(FlagReg::C));
    }
    #[test]
    fn di() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = true;

        let i = Inst::Di;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(false, ime);
    }
    #[test]
    fn ei() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        let i = Inst::Ei;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(true, ime);
    }
    //
    // jump instructions
    //
    #[test]
    fn jp_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        let i = Inst::Jp(0x200);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));
    }
    #[test]
    fn jp_hl() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::HL, 0x200);
        let i = Inst::JpHL;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(1, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));
    }
    #[test]
    fn jp_f_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x0);

        let i = Inst::Jpf(JpFlag::Z, 0x200);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x0, reg.read_reg16(&Reg16::PC));

        reg.set_f(FlagReg::Z);
        let i = Inst::Jpf(JpFlag::Z, 0x200);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));
    }
    #[test]
    fn jr_dd() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        let i = Inst::Jr(-1);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x1ff, reg.read_reg16(&Reg16::PC));
    }
    #[test]
    fn jr_f_dd() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        reg.set_f(FlagReg::C);
        let i = Inst::Jrf(JpFlag::Nc, -1);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));

        reg.clear_f(FlagReg::C);
        let i = Inst::Jrf(JpFlag::Nc, -1);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x1ff, reg.read_reg16(&Reg16::PC));
    }
    #[test]
    fn call_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        reg.write_reg16(&Reg16::SP, 0x1000);
        let i = Inst::Call(0x100);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(6, m);
        assert_eq!(0x100, reg.read_reg16(&Reg16::PC));
        assert_eq!(0xffe, reg.read_reg16(&Reg16::SP));
        assert_eq!(0x200, mem.read_word(0xffe));
    }
    #[test]
    fn call_f_nn() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        reg.write_reg16(&Reg16::SP, 0x1000);
        let i = Inst::Callf(JpFlag::C, 0x100);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(3, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));
        assert_eq!(0x1000, reg.read_reg16(&Reg16::SP));

        reg.set_f(FlagReg::C);
        let i = Inst::Callf(JpFlag::C, 0x100);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(6, m);
        assert_eq!(0x100, reg.read_reg16(&Reg16::PC));
        assert_eq!(0xffe, reg.read_reg16(&Reg16::SP));
        assert_eq!(0x200, mem.read_word(0xffe));
    }
    #[test]
    fn ret() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::SP, 0x1000);
        mem.write_word(0x1000, 0x100);
        let i = Inst::Ret;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x100, reg.read_reg16(&Reg16::PC));
        assert_eq!(0x1002, reg.read_reg16(&Reg16::SP));
    }
    #[test]
    fn ret_f() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        reg.write_reg16(&Reg16::SP, 0x1000);
        mem.write_word(0x1000, 0x100);
        reg.set_f(FlagReg::C);
        let i = Inst::Retf(JpFlag::Nc);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(2, m);
        assert_eq!(0x200, reg.read_reg16(&Reg16::PC));
        assert_eq!(0x1000, reg.read_reg16(&Reg16::SP));

        reg.clear_f(FlagReg::C);
        let i = Inst::Retf(JpFlag::Nc);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(5, m);
        assert_eq!(0x100, reg.read_reg16(&Reg16::PC));
        assert_eq!(0x1002, reg.read_reg16(&Reg16::SP));
    }
    #[test]
    fn reti() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::SP, 0x1000);
        mem.write_word(0x1000, 0x100);
        let i = Inst::Reti;
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x100, reg.read_reg16(&Reg16::PC));
        assert_eq!(0x1002, reg.read_reg16(&Reg16::SP));
        assert_eq!(true, ime);
    }
    #[test]
    fn rst_n() {
        let mut reg = Registers::new();
        let mut mem = TestMemory::new();
        let mut ime = false;

        reg.write_reg16(&Reg16::PC, 0x200);
        reg.write_reg16(&Reg16::SP, 0x1000);
        let i = Inst::Rst(0x38);
        let m = reg.execute(i, &mut mem, &mut ime).unwrap();
        assert_eq!(4, m);
        assert_eq!(0x38, reg.read_reg16(&Reg16::PC));
        assert_eq!(0xffe, reg.read_reg16(&Reg16::SP));
        assert_eq!(0x200, mem.read_word(0xffe));
    }
}
